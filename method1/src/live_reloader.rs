use cxx_qt::{CxxQtType, Threading};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::pin::Pin;

#[cxx_qt::bridge]
pub mod qobject {
	unsafe extern "C++" {
		include!("cxx-qt-lib/qstring.h");
		type QString = cxx_qt_lib::QString;
	}

	#[auto_cxx_name]
	extern "RustQt" {
		#[qobject]
		#[qml_element]
		// #[qproperty(i32, reload_counter)]
		type LiveReloader = super::LiveReloaderRust;

		/// QInvokable to start watching for changes to qml files
		#[qinvokable]
		fn start_watching(self: Pin<&mut LiveReloader>);

		/// QInvokable to reload the QML cache
		#[qinvokable]
		fn reload_qml_cache(self: &LiveReloader);

		/// QSignal to notify QML of a change
		#[qsignal]
		fn qml_changed(self: Pin<&mut LiveReloader>);
	}

	impl cxx_qt::Threading for LiveReloader {}
}

#[derive(Default)]
pub struct LiveReloaderRust {
	// reload_counter: i32,
	is_watching: bool,
}

impl qobject::LiveReloader {
	pub fn start_watching(mut self: Pin<&mut Self>) {
		if self.is_watching {
			return;
		}

		let mut rust_mut = self.as_mut().rust_mut();
		rust_mut.is_watching = true;

		let qt_thread = self.qt_thread();

		std::thread::spawn(move || {
			let (tx, rx) = std::sync::mpsc::channel();
			let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

			// Watch the "qml" folder
			if let Err(e) = watcher.watch(
				Path::new(&format!("{}/qml", env!("CARGO_MANIFEST_DIR"))),
				RecursiveMode::Recursive,
			) {
				println!("Error watching file: {:?}", e);
				return;
			}

			// Handle file modification events
			for res in rx {
				match res {
					Ok(event) => {
						let kind = event.kind;
						if kind.is_modify() || kind.is_create() || kind.is_remove() {
							qt_thread
								.queue(|qobject| {
									qobject.qml_changed();
								})
								.unwrap();
						}
					}
					Err(e) => println!("Watch error: {:?}", e),
				}
			}
		});
	}

	fn reload_qml_cache(self: &Self) {
		crate::engine::ffi::reload_qml_cache();
	}
}
