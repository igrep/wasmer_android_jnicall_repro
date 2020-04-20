#[macro_use]
extern crate lazy_static;
extern crate jni;
#[macro_use]
extern crate log;
extern crate android_logger;

use jni::{
    errors::ErrorKind,
    objects::{GlobalRef, JClass, JObject, JString},
    sys::jbyteArray,
    JNIEnv, JavaVM,
};
use log::Level;
use std::fs::File;
use std::sync::Mutex;
use std::thread;
use wasmer_runtime::{compile, func, imports, ImportObject, Instance, Module};

lazy_static! {
    static ref ENV: Mutex<Option<JavaVM>> = { Mutex::new(None) };
    static ref CLASS: Mutex<Option<GlobalRef>> = { Mutex::new(None) };
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_wasmer_android_MainActivity_JNIExecuteWasm(
    env: JNIEnv,
    _: JClass,
    callback: JObject,
    module_bytes: jbyteArray,
) {
    init_android_logger();

    std::panic::set_hook(Box::new(|panic_info| {
        error!("ERR: {}", panic_info.to_string());
    }));

    // build module
    let module_bytes = env.convert_byte_array(module_bytes).unwrap();
    let main_instance = load_module(&module_bytes);

    // set global variables
    let class = env.new_global_ref(callback).unwrap();
    *CLASS.lock().unwrap() = Some(class);
    let vm = env.get_java_vm().unwrap();
    *ENV.lock().unwrap() = Some(vm);

    // Succeeds
    java_test();
    java_test();

    // fails to call java_test
    main_instance.call("main", &[]).unwrap();

    java_test();
}

fn init_android_logger() {
    android_logger::init_once(android_logger::Config::default().with_min_level(Level::Trace));
}

pub fn load_module(module_bytes: &[u8]) -> Instance {
    // Compile the module.
    let module = compile(&module_bytes).unwrap();
    // Create the ImportObject with our interface imported.
    let import_object = create_import_object(&module);
    // Instantiate the module.
    let module_instance = module.instantiate(&import_object).unwrap();
    module_instance
}

fn create_import_object(_module: &Module) -> ImportObject {
    let import_object = imports! {
        "env" => {
            "Test" => func!(spawn_java_test),
        },
    };
    import_object
}

fn spawn_java_test() {
    thread::spawn(java_test);
}

fn java_test() {
    info!("BEGIN java_test");
    // Get env.
    let ovm = &*ENV.lock().unwrap();
    let vm = ovm.as_ref().unwrap();
    let env = vm.get_env().unwrap();
    // Get the class.
    let o_class = &*CLASS.lock().unwrap();
    let class_ref = o_class.as_ref().unwrap();
    let class = class_ref.as_obj();
    // Call java code.
    info!("      java_test: before calling Java method");
    match env.call_method(*class, "Test", "()V", &[]) {
        Ok(_) => {}
        Err(jerr) => {
            let k = jerr.kind();
            if let ErrorKind::JavaException = k {
                let jex = env.exception_occurred().unwrap();
                env.throw(jex).unwrap();
            } else {
                error!("Unexpected kind of JavaException: {}", k);
            }
        }
    }
}
