// Source: https://github.com/gyroflow/gyroflow/blob/master/src/ui_live_reload.cpp

#include <QDirIterator>
#include <QTimer>
#include <QUrl>
#include <QQmlApplicationEngine>
#include <QQmlComponent>
#include <QQuickItem>
#include <QQuickWindow>
#include <QFileSystemWatcher>
#include <QUrlQuery>
#include <QDateTime>

#include "live_reload.h"

void init_live_reload(QQmlApplicationEngine *engine, const QString &path) {
    QFileSystemWatcher *w = new QFileSystemWatcher();
    QDirIterator it(path, QDirIterator::Subdirectories);
    while (it.hasNext()) {
        it.next();
        auto i = it.fileInfo();
        if (i.isFile())
            w->addPath(i.absoluteFilePath());
    }

    QObject::connect(w, &QFileSystemWatcher::fileChanged, [=](const QString &file) {
        QTimer::singleShot(50, [=] {
            auto wnd = qobject_cast<QQuickWindow *>(engine->rootObjects().first());
            if (!wnd)
                return;

            QObject *loaderObj =
                wnd->findChild<QObject*>("AppLoader", Qt::FindChildrenRecursively);
            if (!loaderObj) {
                qWarning() << "AppLoader not found!";
                return;
            }

            // Clear QML cache
            engine->clearComponentCache();

            // Build the canonical, absolute form of the path
            QUrl mainPath = QUrl::fromLocalFile(path + "/App.qml");
            QUrlQuery cacheQuery;
            cacheQuery.addQueryItem("t", QString::number(QDateTime::currentMSecsSinceEpoch()));
            mainPath.setQuery(cacheQuery);

            // Force reload
            loaderObj->setProperty("source", QVariant());
            loaderObj->setProperty("source", mainPath);
        });
    });
}