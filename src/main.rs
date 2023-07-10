#[repr(C)]
struct RustObject {
    a: i32,
}

extern "C" fn callback(target: *mut RustObject, a: i32) {
    println!("I'm called from C with value {a}");
    unsafe {
        (*target).a = a;
    }
}

#[link(name = "callbacks")]
extern "C" {
    static test_global: libc::c_int;
    fn register_callback(target: *mut RustObject, cb: extern "C" fn(*mut RustObject, i32)) -> i32;
    fn trigger_callback();
}

fn main() {
    println!("Test global value {}", unsafe { test_global as i32 });

    let mut rust_object = Box::new(RustObject { a: 5 });

    println!("My value was {}", rust_object.a);

    unsafe {
        register_callback(&mut *rust_object, callback);
        trigger_callback();
    }

    println!("Now my value is {}", rust_object.a);
}
