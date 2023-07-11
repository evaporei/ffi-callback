use std::ptr::NonNull;

#[repr(C)]
struct Mango {
    b: libc::c_int,
}

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

    fn new_mango() -> Option<NonNull<Mango>>;
    fn print_mango(mango: Option<NonNull<Mango>>);
    fn free_mango(mango: Option<NonNull<Mango>>);
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

    println!();

    mango_fun();
}

fn mango_fun() {
    let mango = unsafe { new_mango() };

    unsafe { print_mango(None) };
    unsafe { print_mango(mango) };

    if let Some(mut mango) = mango {
        unsafe { mango.as_mut() }.b = 44;
    }

    unsafe { print_mango(mango) };

    unsafe { free_mango(mango) };
    unsafe { free_mango(None) };
}
