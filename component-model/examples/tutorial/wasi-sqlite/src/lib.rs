#[allow(warnings)]
mod bindgen;

use bindgen::exports::wasi::sqlite::sqlite;
use bindgen::exports::wasi::sqlite::sqlite_vec;

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct WasiSqliteComponent;

impl sqlite::Guest for WasiSqliteComponent {
    fn hello(name: String) {
        println!("hello :{}", name);
    }
}

impl sqlite_vec::Guest for WasiSqliteComponent {
    fn init(x: u32, y: u32) -> u32 {
        x + y + 1
    }
}
