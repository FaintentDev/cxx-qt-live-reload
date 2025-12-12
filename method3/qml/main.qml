import QtQuick

import org.cxx_qt_live_reload

Window {
	width: 640
	height: 480
	visible: true
	title: "CXX Qt Live Reload"

	FileWatcher {
		id: fileWatcher
		onQuitRequested: Qt.quit();
		Component.onCompleted: fileWatcher.startWatching()
	}

	App {}

	// Optional: Visual indicator that reload happened
	Text {
		anchors.bottom: parent.bottom
		text: "Reloads: " + fileWatcher.reloadCounter
	}
}