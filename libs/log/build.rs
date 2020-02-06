// We will need to compile and link against the exposed functions from C

use bindgen::{

};

const C_HEADER: &'static str = r#"
#include <android/log.h>
"#;

fn main() {
    println!("cargo:rustc-link-lib=liblog");

    let bindings = bindgen::builder()
        .header_contents("header.h", C_HEADER)
        // Set the location of the headers
        .clang_arg()
}
