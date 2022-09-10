use std::{fs, env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=hello-world.rs.rc");
    println!("cargo:rerun-if-changed=hello-world.rs.exe.manifest");
    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        let rc_template = fs::read_to_string("hello-world.rs.exe.manifest").unwrap();
        let arch = match &*env::var("CARGO_CFG_TARGET_ARCH").unwrap() {
            "x86" => "x86",
            "x86_64" => "amd64",
            "arm" => "arm",
            "aarch64" => "arm64",
            _ => "",
        };
        let mut version = env::var("CARGO_PKG_VERSION").unwrap();
        let last_dot = version.rfind(".").unwrap();
        version.insert_str(last_dot, ".0"); // windows style
        let modified = rc_template.replace("__VERSION__", &version).replace("__ARCHITECTURE__", arch);
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        fs::write(out_dir.join("hello-world.rs.exe.manifest"), modified).unwrap();
        fs::copy("hello-world.rs.rc", out_dir.join("hello-world.rs.rc")).unwrap();
        env::set_current_dir(out_dir).unwrap();
        embed_resource::compile("hello-world.rs.rc");
    }
    // env::set_current_dir(env::var_os("CARGO_MANIFEST_DIR").unwrap()).unwrap();
}
