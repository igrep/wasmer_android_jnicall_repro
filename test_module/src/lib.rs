#[link(wasm_import_module = "env")]
extern "C" {
    pub fn Test();
}

#[no_mangle]
pub fn main() {
    unsafe {
        Test();
    }
}

