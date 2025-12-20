import QtQuick

import org.cxx_qt_live_reload

Window {
	width: 640
	height: 480
	visible: true
	title: "CXX Qt Live Reload"

	AppReloader {
		id: appReloader
		active: true
		source: "App.qml"
		onReload: {
			active = false
			source = ""
			Qt.callLater(refreshLoader)
		}
	}

	Loader {
		id: loader
		anchors.fill: parent
		active: appReloader.active
		source: appReloader.source
	}

	Text {
		anchors.bottom: parent.bottom
		text: "Reloads: " + appReloader.counter
	}
}
