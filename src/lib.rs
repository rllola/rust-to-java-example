// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass};

#[no_mangle]
// do I need "system" ?
// pub extern "system" fn
pub extern fn Java_HelloWorld_hello_1world(_env: JNIEnv, _class: JClass) {
    println!("Hello, world!");
}
