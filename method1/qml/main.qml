import QtQuick

import org.cxx_qt_live_reload

Window {
	width: 640
	height: 480
	visible: true
	title: "CXX Qt Live Reload"

	LiveReloader {
		id: reloader
		onQmlChanged: contentLoader.reload()
		// Start watching immediately
		Component.onCompleted: reloader.startWatching()
	}

	Loader {
		id: contentLoader
		anchors.fill: parent
		source: "App.qml"

		property int counter: 0

		function reload() {
			console.log("Reloading QML...")
			source = ""
			reloader.reloadQmlCache()
			source = "App.qml"
			counter++
		}
	}

	// Optional: Visual indicator that reload happened
	Text {
		anchors.bottom: parent.bottom
		text: "Reloads: " + contentLoader.counter
	}
}