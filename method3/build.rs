use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
	CxxQtBuilder::new()
		.qml_module(QmlModule {
			uri: "org.cxx_qt_live_reload",
			rust_files: &["src/file_watcher.rs"],
			qml_files: &["qml/main.qml", "qml/App.qml"],
			..Default::default()
		})
		.build();
}
