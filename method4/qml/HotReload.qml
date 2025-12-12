import QtQuick 2.11
import QtQuick.Controls 2.4
import QtQuick.Layouts 1.11
import QtQuick.Window 2.11

import org.chess_studio 1.0

Window {
	id: window
	width: 1000
	height: 1000
	visible: true

	Reloader {
		id: reloader
	}

	Loader {
		id: loader
		anchors.fill: parent
		source: "file:///<REPO_DIRECTORY>/method4/qml/MainComponent.qml?t=" + Date.now()

		function clearCacheAndReload() {
			reloader.clearCache()
			source = "file:///<REPO_DIRECTORY>/method4/qml/MainComponent.qml?t=" + Date.now()
			active = true
		}

		function reload() {
			active = false
			source = ""
			Qt.callLater(clearCacheAndReload)
		}

		onStatusChanged: {
			if (loader.status === Loader.Error || loader.status === Loader.Null) {
			} else {
				if (item) {
					if (item.implicitWidth !== "undefined" && item.implicitWidth > window.width)
						window.width = Math.min(item.implicitWidth, Screen.width)
					if (item.implicitHeight !== "undefined" && item.implicitHeight > window.height)
						window.height = Math.min(item.implicitHeight, Screen.height)
				}
			}
		}
	}

	Shortcut {
		sequence: "F5"
		onActivated: loader.reload()
	}
}
