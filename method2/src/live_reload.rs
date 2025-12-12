#[cxx::bridge]
pub mod ffi {
	unsafe extern "C++" {
		include!("cxx-qt-lib/qqmlapplicationengine.h");
		type QQmlApplicationEngine = cxx_qt_lib::QQmlApplicationEngine;

		include!("cxx-qt-lib/qstring.h");
		type QString = cxx_qt_lib::QString;

		include!("live_reload.h");
		unsafe fn init_live_reload(engine: *mut QQmlApplicationEngine, path: &QString);
	}
}
