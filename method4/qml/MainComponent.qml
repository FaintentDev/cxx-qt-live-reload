import QtQuick

Rectangle {
	visible: true
	color: "lightblue" // Change this color and save to test!
	anchors.fill: parent

	Text {
		id: msg
		anchors.centerIn: parent
		text: "Change me and save!"
		font.pixelSize: 24
	}

	OtherComponent {
		anchors {
			horizontalCenter: msg.horizontalCenter
			top: msg.bottom
			topMargin: 10
		}
		text: "Update me!"
	}
}
