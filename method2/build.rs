use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
	CxxQtBuilder::new()
		.cc_builder(|cc| {
			cc.file("src/cpp/live_reload.cpp");
			cc.include("src/cpp");
		})
		.file("src/live_reload.rs")
		.qml_module(QmlModule {
			uri: "org.cxx_qt_live_reload",
			rust_files: &["src/counter.rs"],
			qml_files: &["qml/main.qml", "qml/App.qml"],
			..Default::default()
		})
		.qt_module("Quick") // for QQuickItem and QQuickWindow to resolve in live_reload.cpp
		.build();
}
