use bindgen;
use cc;
use std::path::Path;

fn main() {

    cc::Build
        ::new()        
        .include(Path::new("C:/projects/RUST/clap-wrap/clap-main/include/clap"))
        .includes(Path::new("C:/projects/RUST/clap-wrap/clap-main/src"))
        .try_compile("clapc");

    let bindings = 
        bindgen::Builder
            ::default()
            .header("wrapper.h")
            .generate()
            .expect("Failed to generate bindings");

    bindings   
        .write_to_file(Path::new("C:/projects/RUST/clap-wrap/src/bindings.rs"))
        .expect("Failed to write bindings");

}