use std::path::{Path, PathBuf};
use bindgen::CargoCallbacks;

fn main() {

    let mut files = Vec::new();
    recursion(&mut files, "silk/interface").unwrap();
    recursion(&mut files, "silk/src").unwrap();
    println!("cargo:rustc-link-lib=static=silk");

    cc::Build::new()
        .includes(["silk/src", "silk/interface"])
        .files(files)
        .compile("silk");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("silk/interface/SKP_Silk_SDK_API.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src").join("SKP_Silk_SDK_API_BINDINGS.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn recursion<P: AsRef<Path>>(v: &mut Vec<String>, dir: P) -> std::io::Result<()> {
    let rd = std::fs::read_dir(dir)?;
    for x in rd {
        let de = x?;
        let path = de.path();
        if path.is_dir() {
            recursion(v, path)?;
        } else {
            let path = path.into_os_string().into_string().unwrap();
            if path.ends_with(".c") {
                v.push(path);
            }
        }
    }
    Ok(())
}