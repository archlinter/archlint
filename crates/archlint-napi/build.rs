extern crate napi_build;

fn main() {
    napi_build::setup();

    // For macOS Node addon builds: allow unresolved symbols to be resolved by the host (node).
    // Scope to cdylib only to avoid affecting test executables/proc-macros in the workspace.
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    {
        #[cfg(target_os = "macos")]
        {
            println!("cargo:rustc-cdylib-link-arg=-undefined");
            println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
        }
    }
}
