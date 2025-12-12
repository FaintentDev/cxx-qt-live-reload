use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};

mod counter;
mod live_reload;

fn main() {
	let mut app = QGuiApplication::new();
	let mut engine = QQmlApplicationEngine::new();

	let ui_path = QString::from(format!("{}/qml", env!("CARGO_MANIFEST_DIR")));
	unsafe {
		live_reload::ffi::init_live_reload(engine.as_mut_ptr(), &ui_path);
	}

	if let Some(engine) = engine.as_mut() {
		engine.load(&QUrl::from("qrc:/qt/qml/org/cxx_qt_live_reload/qml/main.qml"));
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
