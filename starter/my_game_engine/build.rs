use std::path::PathBuf;

fn main() {
    // Compile the C code itself
    std::env::set_var("CC", "gcc");

    std::env::set_var("MACOSX_DEPLOYMENT_TARGET", "11.0");

    println!("cargo:rustc-link-search=native=../glfw-3.4/lib-arm64");
    println!("cargo:rustc-link-lib=glfw3");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=OpenGL");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=GLUT");
    
    cc::Build::new()
        .compiler("gcc") // make sure to us gcc instead of clang on macos
        .file("../opengl_wrapper_lib/opengl_wrapper_lib.c")
        .include("../opengl_wrapper_lib/")
        // .include("../glfw-3.4/include")
        // .flag("-L../glfw-3.4/lib-arm64/ -lglfw.3 -framework OpenGL -framework GLUT")
        .compile("opengl_wrapper_lib");
}


// from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
// TODO: fix the code below so it can find the glut header when using clang compiler
// fn _generate_ffi(){

//     // Tell cargo to look for shared libraries in the specified directory
//     println!("cargo:rustc-link-search=../glfw-3.4/lib-arm64/");

//     // Tell cargo to tell rustc to link the system bzip2
//     // shared library.
//     println!("cargo:rustc-link-lib=bz2");
//     println!("cargo:rustc-link-lib=glfw.3");

//     // The bindgen::Builder is the main entry point
//     // to bindgen, and lets you build up options for
//     // the resulting bindings.
//     let bindings = bindgen::Builder::default()
//         // The input header we would like to generate
//         // bindings for.
//         .header("wrapper.h")
//         .clang_arg("-iframework /System/Library/Frameworks/OpenGL")
//         .clang_arg("-I/opt/homebrew/include")
//         // Tell cargo to invalidate the built crate whenever any of the
//         // included header files changed.
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
//         // Finish the builder and generate the bindings.
//         .generate()
//         // Unwrap the Result and panic on failure.
//         .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     let out_path = PathBuf::from("./src/");
//     bindings
//         .write_to_file(out_path.join("bindings_gen.rs"))
//         .expect("Couldn't write bindings!");

// }

