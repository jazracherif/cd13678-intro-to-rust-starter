fn main() {
    // Compile the C code itself
    cc::Build::new()
        .compiler("gcc") // make sure to us gcc instead of clang on macos
        .file("../opengl_wrapper_lib/opengl_wrapper_lib.c")
        .include("../opengl_wrapper_lib/")
        .compile("opengl_wrapper_lib");

    println!("cargo::rustc-link-lib=glfw");
    println!("cargo::rustc-link-lib=GL");
    println!("cargo::rustc-link-lib=glut");

}