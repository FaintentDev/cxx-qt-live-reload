#[cxx::bridge]
pub mod ffi {
	unsafe extern "C++" {
		include!("cxx-qt-lib/qqmlapplicationengine.h");
		type QQmlApplicationEngine = cxx_qt_lib::QQmlApplicationEngine;

		include!("engine_ext.h");

		unsafe fn set_global_engine(engine: *mut QQmlApplicationEngine);
		fn reload_qml_cache();
	}
}
