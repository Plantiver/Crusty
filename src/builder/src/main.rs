use std::process::Command;

fn main() {
    println!("Step 1: Compiling the OS Kernel...");

    let status = Command::new("cargo")
        .args([
            "build",
            "-p",
            "kernel", // Selects just the kernel crate
            "--target",
            "x86_64-unknown-none", // Compiles for bare-metal x86_64
            "--release",
        ])
        .status()
        .expect("Failed to execute cargo build");

    if !status.success() {
        std::process::exit(1);
    }

    println!("Step 2: Locating kernel binary and building Limine ISO... TODO");
}
