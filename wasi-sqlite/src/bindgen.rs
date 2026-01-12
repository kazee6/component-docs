// The line below will be expanded as Rust code containing
wit_bindgen::generate!({
    path: "wit/world.wit",
});

// In the lines below we use the generated `export!()` macro re-use and
use super::WasiSqliteComponent;
export!(WasiSqliteComponent);