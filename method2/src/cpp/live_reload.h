#pragma once

#include <QQmlApplicationEngine>
#include <QString>

// Expose the live reload initialization function so it can be used from other
// translation units (and from Rust via cxx/cxx-qt bindings).
void init_live_reload(QQmlApplicationEngine *engine, const QString &path);