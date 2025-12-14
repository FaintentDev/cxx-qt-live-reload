Method 1: uses a file watcher on the rust side to notify of changes then updates the Loader's source
(see [app_reloader.rs](method1/src/app_reloader.rs))

Method 2: this is for you C++ monkeys out there
(see [live_reload.cpp](method2/src/cpp/live_reload.cpp))

Method 3: instead of clearing the cache it reloads the entire app upon quit in a loop
(see [main.rs](method3/src/main.rs))

Method 4: props to David Jobet on Zulip for this method (very similar to method1) - press F5 to see changes. must replace <REPO_DIRECTORY> in order for this to workD
(see [reloader.rs](method4/src/reloader.rs), [HotReload.qml](method4/qml/HotReload.qml))

## The Basis of QML Live Reloading

- clear the component cache before updating the Loader component
- reset the 'source' property of Loader to a file url (either set by Rust, C++, or the full absolute path in QML) - here the use of the "?t=..." query is purely for cache busting and WILL NOT work without it
