import QtQuick

Rectangle {
	id: root

	property string text

	// change color here and see updates
	color: "red"
	width: inner.implicitWidth
	height: inner.implicitHeight

	Text {
		id: inner
		anchors.fill: parent
		text: root.text
		font.pixelSize: 24
	}
}
