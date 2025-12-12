use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

mod engine;
mod live_reloader;

fn main() {
	let mut app = QGuiApplication::new();
	let mut engine = QQmlApplicationEngine::new();

	unsafe {
		engine::ffi::set_global_engine(engine.as_mut_ptr());
	}

	if let Some(engine) = engine.as_mut() {
		let url = QUrl::from_local_file(&format!("{}/qml/main.qml", env!("CARGO_MANIFEST_DIR")).into());
		engine.load(&url);
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
}
