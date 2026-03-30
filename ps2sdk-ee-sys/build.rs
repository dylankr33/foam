use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    let ps2sdk_str = env::var("PS2SDK")?;
    let ps2dev = env::var("PS2DEV")?;
    println!("cargo:rustc-link-search={ps2sdk_str}/ee/lib");
    println!("cargo:rustc-link-search={ps2dev}/ee/mips64r5900el-ps2-elf/lib");
    println!("cargo:rustc-link-lib=kernel");
    println!("cargo:rustc-link-lib=cglue");
    println!("cargo:rustc-link-lib=cdvd");
    println!("cargo:rustc-link-lib=c");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args([
            "-nostdinc".to_string(),
            format!("-I{ps2dev}/ee/mips64r5900el-ps2-elf/include"),
            format!("-I{ps2dev}/ee/lib/gcc/mips64r5900el-ps2-elf/15.2.0/include"),
            format!("-I{ps2sdk_str}/common/include"),
            format!("-I{ps2sdk_str}/ee/include"),
            "-D_EE".to_string(),
            "--target=mips64el-none-elf".to_string(),
            "-mabi=n32".to_string(),
        ])
        .ctypes_prefix("core::ffi")
        .use_core()
        .layout_tests(false)
        .generate()
        .expect("Could not generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    Ok(())
}
