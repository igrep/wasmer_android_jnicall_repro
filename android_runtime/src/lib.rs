#[macro_use]
extern crate lazy_static;
extern crate jni;

use jni::{
    objects::{GlobalRef, JClass, JObject},
    sys::jbyteArray,
    JNIEnv, JavaVM,
};
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
    let module_bytes = env.convert_byte_array(module_bytes).unwrap();
    let main_instance = load_module(&module_bytes);

    // Succeeds
    java_test_acual(&env, &callback);

    let class = env.new_global_ref(callback).unwrap();
    *CLASS.lock().unwrap() = Some(class);
    let vm = env.get_java_vm().unwrap();
    *ENV.lock().unwrap() = Some(vm);

    // but the call inside wasm fails
    main_instance.call("main", &[]).unwrap();

    java_test_acual(&env, &callback);
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

fn java_test(_ctx: &mut Ctx) {
    // Get env.
    let ovm = &*ENV.lock().unwrap();
    let vm = ovm.as_ref().unwrap();
    let env = vm.get_env().unwrap();
    // Get the class.
    let o_class = &*CLASS.lock().unwrap();
    let class_ref = o_class.as_ref().unwrap();
    let class = class_ref.as_obj();
    // Call java code.
    java_test_acual(&env, &class);
}

fn java_test_acual(env: &JNIEnv, class: &JObject) {
    env.call_method(*class, "Test", "()V", &[])
        .unwrap();
}
