use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
	CxxQtBuilder::new()
		.cc_builder(|cc| {
			cc.file("src/cpp/engine_ext.cpp");
			cc.include("src/cpp");
		})
		.file("src/engine_ext.rs")
		.qml_module(QmlModule {
			uri: "org.cxx_qt_live_reload",
			rust_files: &["src/app_reloader.rs"],
			qml_files: &["qml/main.qml", "qml/App.qml"],
			..Default::default()
		})
		.build();
}
