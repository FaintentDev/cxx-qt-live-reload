#include "src/qt_extra.h"

void qml_clear_component_cache(QQmlApplicationEngine& engine) {
	engine.clearComponentCache();
}
