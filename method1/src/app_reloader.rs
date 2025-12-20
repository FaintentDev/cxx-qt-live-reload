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
		#[qproperty(bool, active)]
		#[qproperty(QUrl, source)]
		#[qproperty(i32, counter)]
		type AppReloader = super::AppReloaderRust;

		#[qsignal]
		fn reload(self: Pin<&mut AppReloader>);

		#[qinvokable]
		fn refresh_loader(self: Pin<&mut AppReloader>);
	}

	impl cxx_qt::Initialize for AppReloader {}
	impl cxx_qt::Threading for AppReloader {}
}

#[derive(Default)]
pub struct AppReloaderRust {
	active: bool,
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
							// Skip consecutive events within 100ms
							if last_handled.elapsed() < Duration::from_millis(100) {
								continue;
							}

							// Ensure changes are saved
							thread::sleep(Duration::from_millis(100));
							last_handled = Instant::now();

							qt_thread
								.queue(|qobject| {
									qobject.reload();
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

impl qobject::AppReloader {
	pub fn refresh_loader(mut self: Pin<&mut Self>) {
		// Clear C++ Cache
		crate::engine_ext::ffi::reload_qml_cache();

		// Set the source as a local file
		let mut src = QUrl::from_local_file(&format!("{}/qml/App.qml", env!("CARGO_MANIFEST_DIR")).into());
		// Bust the cache with a query
		src.set_query(&format!("t={}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()).into());

		// Re-activate the Loader
		self.as_mut().set_source(src);
		self.as_mut().set_active(true);

		// Increment the counter
		let next = self.counter() + 1;
		self.set_counter(next);
	}
}
