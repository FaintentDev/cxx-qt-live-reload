import QtQuick

import org.cxx_qt_live_reload

Window {
	width: 640
	height: 480
	visible: true
	title: "CXX Qt Live Reload"

	AppReloader {
		id: appReloader
		source: "App.qml"
	}

	Loader {
		id: loader
		anchors.fill: parent
		source: appReloader.source
	}

	Text {
		anchors.bottom: parent.bottom
		text: "Reloads: " + appReloader.counter
	}
}