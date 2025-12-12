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
        // opacity: appLoader.status == Loader.Ready? 1 : 0.5;
        // NumberAnimation on opacity { }
        onStatusChanged: {
            if (status == Loader.Ready) {
                // Qt.callLater(item.isMobileLayoutChanged);
                // Qt.callLater(item.isLandscapeChanged);
            }
        }
        sourceComponent: Component {
            App { objectName: "App"; }
        }
    }

	// Optional: Visual indicator that reload happened
	Text {
		anchors.bottom: parent.bottom
		text: "Reloads: " + counter.reloadCounter
	}
}