use std::pin::Pin;

use qobject::qml_clear_component_cache;

use crate::ENGINE;

#[cxx_qt::bridge]
mod qobject {
	unsafe extern "C++" {
		include!(<cxx-qt-lib/qqmlapplicationengine.h>);
		type QQmlApplicationEngine = cxx_qt_lib::QQmlApplicationEngine;

		include!("src/qt_extra.h");
		fn qml_clear_component_cache(engine: Pin<&mut QQmlApplicationEngine>);
	}

	unsafe extern "RustQt" {
		#[qobject]
		#[qml_element]
		type Reloader = super::ReloaderRust;

		#[qinvokable]
		#[rust_name = "clear_cache"]
		fn clearCache(self: Pin<&mut Reloader>);
	}
}

#[derive(Default)]
pub struct ReloaderRust {}

impl qobject::Reloader {
	fn clear_cache(self: Pin<&mut Self>) {
		let engine = ENGINE.load(std::sync::atomic::Ordering::Relaxed);
		if let Some(engine) = unsafe { engine.as_mut() } {
			let mut engine = unsafe { Pin::new_unchecked(engine) };
			qml_clear_component_cache(engine.as_mut());
		}
	}
}
