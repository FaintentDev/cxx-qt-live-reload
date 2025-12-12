use std::pin::Pin;

#[cxx_qt::bridge]
mod qobject {
	#[auto_cxx_name]
	extern "RustQt" {
		#[qobject]
		#[qml_element]
		#[qproperty(i32, reload_counter)]
		type ReloadCounter = super::ReloadCounterRust;
	}

	impl cxx_qt::Initialize for ReloadCounter {}
}

#[derive(Default)]
pub struct ReloadCounterRust {
	reload_counter: i32,
}

impl cxx_qt::Initialize for qobject::ReloadCounter {
	fn initialize(self: Pin<&mut Self>) {
		self.on_reload_counter_changed(|qobject| {
			println!("Reload count changed to {}", qobject.reload_counter());
		})
		.release();
	}
}
