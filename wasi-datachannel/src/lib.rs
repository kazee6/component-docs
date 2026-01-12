#[allow(warnings)]
mod bindgen;

use bindgen::exports::wasi::datachannel::sqlite;

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct WasiDataChannelComponent;

impl sqlite::Guest for WasiDataChannelComponent {
    fn hello(name: String) {
        println!("hello :{}", name);
    }
}