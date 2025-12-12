Method 1: **broken** (see [live_reloader.rs](method1/src/live_reloader.rs))

Method 2: only reveals changes once and App.qml's new contents overlaps main.qml's for some unknown reason (see [live_reload.cpp](method2/src/cpp/live_reload.cpp))

Method 3: instead of clearing the cache it reloads the entire app upon quit in a loop (see [main.rs](method3/src/main.rs))
