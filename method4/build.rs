use cxx_qt_build::{CxxQtBuilder, QmlModule};

#[cfg(debug_assertions)]
fn qml_files() -> &'static [&'static str] {
	&["qml/HotReload.qml"]
}

#[cfg(not(debug_assertions))]
fn qml_files() -> &'static [&'static str] {
	&["qml/Main.qml", "qml/MainComponent.qml", "qml/OtherComponent.qml"]
}

fn main() {
	CxxQtBuilder::new()
		.qml_module(QmlModule {
			uri: "org.chess_studio",
			qml_files: qml_files(),
			rust_files: &["src/reloader.rs"],
			..Default::default()
		})
		.cc_builder(|builder| {
			builder.file("src/qt_extra.cc");
			builder.include(".");
		})
		.build();
}
