fn main() {
    println!("cargo:rerun-if-changed=src/hello.c");
    cc::Build::new()
        .file("callbacks.c")
        .compile("callbacks");
}
