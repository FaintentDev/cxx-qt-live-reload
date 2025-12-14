use cxx_qt::Threading;
use cxx_qt_lib::QUrl;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::pin::Pin;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::{sync, thread};

#[cxx_qt::bridge]
pub mod qobject {
	unsafe extern "C++" {
		include!("cxx-qt-lib/qurl.h");
		type QUrl = cxx_qt_lib::QUrl;
	}

	#[auto_cxx_name]
	extern "RustQt" {
		#[qobject]
		#[qml_element]
		#[qproperty(QUrl, source)]
		#[qproperty(i32, counter)]
		type AppReloader = super::AppReloaderRust;
	}

	impl cxx_qt::Initialize for AppReloader {}
	impl cxx_qt::Threading for AppReloader {}
}

#[derive(Default)]
pub struct AppReloaderRust {
	source: QUrl,
	counter: i32,
}

impl cxx_qt::Initialize for qobject::AppReloader {
	fn initialize(self: Pin<&mut Self>) {
		let qt_thread = self.qt_thread();

		thread::spawn(move || {
			let (tx, rx) = sync::mpsc::channel();
			let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

			// Watch the "qml" folder
			let qml_path = PathBuf::from(format!("{}/qml", env!("CARGO_MANIFEST_DIR")));
			if let Err(e) = watcher.watch(&qml_path, RecursiveMode::Recursive) {
				println!("Error watching QML: {:?}", e);
				return;
			}

			// Handle file modification events
			let mut last_handled = Instant::now();
			for res in rx {
				match res {
					Ok(event) => {
						let kind = event.kind;
						if kind.is_modify() || kind.is_create() || kind.is_remove() {
							if last_handled.elapsed() < Duration::from_millis(50) {
								continue;
							}
							last_handled = Instant::now();
							qt_thread
								.queue(|mut qobject| {
									// Clear C++ Cache
									crate::engine_ext::ffi::reload_qml_cache();
									// Set the new source
									let mut main_path = QUrl::from_local_file(&"method1/qml/App.qml".into());
									// let mut main_path = QUrl::from_local_file(&qml_path.join("App.qml").into());
									main_path.set_query(
										&format!(
											"t={}",
											SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
										)
										.into(),
									);
									qobject.as_mut().set_source(main_path);
									// Increment counter
									let next = qobject.counter() + 1;
									qobject.set_counter(next);
								})
								.unwrap();
						}
					}
					Err(e) => println!("Watch error: {:?}", e),
				}
			}
		});
	}
}
