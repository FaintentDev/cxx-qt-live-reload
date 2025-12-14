#include "engine_ext.h"

// Global pointer to the engine
static QQmlApplicationEngine* g_engine = nullptr;

void set_global_engine(QQmlApplicationEngine* engine) {
	g_engine = engine;
}

void reload_qml_cache() {
	if (g_engine) {
		g_engine->clearComponentCache();
		qDebug() << "Refreshed QML cache!";
	}
}
