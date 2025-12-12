use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

mod file_watcher;

static SHOULD_RELOAD: AtomicBool = AtomicBool::new(false);
static RELOAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
	loop {
		let mut app = QGuiApplication::new();
		let mut engine = QQmlApplicationEngine::new();

		if let Some(engine) = engine.as_mut() {
			#[cfg(debug_assertions)]
			{
				let qml_path = format!("{}/qml/main.qml", env!("CARGO_MANIFEST_DIR"));
				engine.load(&QUrl::from_local_file(&qml_path.into()));
			}
			#[cfg(not(debug_assertions))]
			{
				engine.load(&QUrl::from("qrc:/qt/qml/org/cxx_qt_live_reload/qml/main.qml"));
			}
		}
		if let Some(engine) = engine.as_mut() {
			engine
				.as_qqmlengine()
				.on_quit(|_| {
					println!("QML Quit!");
				})
				.release();
		}

		if let Some(app) = app.as_mut() {
			app.exec();
		}

		// Check if we should reload the app
		if !SHOULD_RELOAD.load(Ordering::SeqCst) {
			break;
		}
		SHOULD_RELOAD.store(false, Ordering::SeqCst);
		RELOAD_COUNTER.fetch_add(1, Ordering::SeqCst);
	}
}
