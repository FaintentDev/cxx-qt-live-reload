import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window

import org.chess_studio 1.0

ApplicationWindow {
	title: qsTr("Chess studio")
	visible: true
	height: 480
	width: 640
	color: "#e4af79"

	MainComponent {
		anchors.fill: parent
	}
}
