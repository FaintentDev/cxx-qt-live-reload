use cxx_qt::{CxxQtType, Threading};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

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
		#[qproperty(usize, reload_counter, READ, CONSTANT)]
		type FileWatcher = super::FileWatcherRust;

		/// QInvokable to start watching for changes to qml files
		#[qinvokable]
		fn start_watching(self: Pin<&mut FileWatcher>);

		/// QSignal to notify QML of a change
		#[qsignal]
		fn quit_requested(self: Pin<&mut FileWatcher>);
	}

	impl cxx_qt::Threading for FileWatcher {}
}

pub struct FileWatcherRust {
	reload_counter: usize,
	is_watching: bool,
}

impl Default for FileWatcherRust {
	fn default() -> Self {
		Self {
			reload_counter: crate::RELOAD_COUNTER.load(Ordering::SeqCst),
			is_watching: false,
		}
	}
}

impl qobject::FileWatcher {
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
			let mut last_handled = Instant::now();
			for res in rx {
				match res {
					Ok(event) => {
						let kind = event.kind;
						// Skip events that arrive too soon after the previous one.
						if last_handled.elapsed() < Duration::from_millis(50) {
							continue;
						}
						last_handled = Instant::now();
						if kind.is_modify() || kind.is_create() || kind.is_remove() {
							let _ = qt_thread.queue(|qobject| {
								// Set the SHOULD_RELOAD flag so that the app relaunches
								crate::SHOULD_RELOAD.store(true, Ordering::SeqCst);

								qobject.quit_requested();
							});
						}
					}
					Err(e) => println!("Watch error: {:?}", e),
				}
			}
		});
	}
}
