import QtQuick

import org.cxx_qt_live_reload

Window {
	width: 640
	height: 480
	visible: true
	title: "CXX Qt Live Reload"

	ReloadCounter {
		id: counter
	}

	Loader {
		id: appLoader;
		objectName: "AppLoader";
		anchors.fill: parent;
		asynchronous: true;
		source: "App.qml"
		// Optional: for the reload counter
        onStatusChanged: {
            if (status == Loader.Ready) {
				counter.reloadCounter++;
            }
        }
	}

	// Optional: Visual indicator that reload happened
	Text {
		anchors.bottom: parent.bottom
		text: "Reloads: " + counter.reloadCounter
	}
}