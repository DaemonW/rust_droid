mod native;
pub use native::{add, say_hello};

use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_tool_rust_NativeUtils_sayHello(env: JNIEnv, _class: JClass) -> jstring {
    let s = say_hello() + test().as_str();
    let response = env.new_string(s).expect("Couldn't create java string!");
    response.into_inner()
}

#[no_mangle]
pub extern "C" fn test() -> String {
    String::from("native message!")
}
