#[cfg(debug_assertions)]
use std::pin::Pin;

use std::sync::atomic::AtomicPtr;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

mod reloader;

static ENGINE: AtomicPtr<QQmlApplicationEngine> = AtomicPtr::new(std::ptr::null_mut());

fn main() {
	let mut app = QGuiApplication::new();

	#[cfg(not(debug_assertions))]
	{
		let mut engine = QQmlApplicationEngine::new();
		if let Some(engine) = engine.as_mut() {
			engine.load(&QUrl::from("qrc:/qt/qml/org/chess_studio/qml/Main.qml"));
		}

		if let Some(app) = app.as_mut() {
			app.exec();
		}
	}

	#[cfg(debug_assertions)]
	{
		let engine = QQmlApplicationEngine::new();
		ENGINE.store(engine.into_raw(), std::sync::atomic::Ordering::Relaxed);
		let engine = ENGINE.load(std::sync::atomic::Ordering::Relaxed);

		if let Some(engine) = unsafe { engine.as_mut() } {
			let engine = unsafe { Pin::new_unchecked(engine) };
			engine.load(&QUrl::from("qrc:/qt/qml/org/chess_studio/qml/HotReload.qml"));
		}

		if let Some(app) = app.as_mut() {
			app.exec();
		}
	}
}
