#[macro_use]
extern crate lazy_static;
extern crate jni;
#[macro_use]
extern crate log;
extern crate android_logger;

use android_logger::Config;
use jni::{
    errors::ErrorKind,
    objects::{GlobalRef, JClass, JObject},
    sys::jbyteArray,
    JNIEnv, JavaVM,
};
use log::Level;
use std::sync::Mutex;
use wasmer_runtime::{compile, func, imports, Ctx, ImportObject, Instance, Module};

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
    android_logger::init_once(Config::default().with_min_level(Level::Trace));

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
            "Test" => func!(java_test),
        },
    };
    import_object
}

fn java_test() {
    // Get env.
    let ovm = &*ENV.lock().unwrap();
    let vm = ovm.as_ref().unwrap();
    let env = vm.get_env().unwrap();
    // Get the class.
    let o_class = &*CLASS.lock().unwrap();
    let class_ref = o_class.as_ref().unwrap();
    let class = class_ref.as_obj();
    // Call java code.
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
