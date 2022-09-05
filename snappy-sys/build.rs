#![cfg(target_os = "linux")]

fn main() {
    build_snappy();
}

fn build_snappy() {
    let dst = cmake::Config::new("snappy").build_target("snappy").build();
    let build = dst.join("build");

    println!("cargo:root={}", build.display());
    println!("cargo:rustc-link-lib=static=snappy");
    println!("cargo:rustc-link-search=native={}", build.display());
    println!("cargo:rustc-link-lib=stdc++");
}
