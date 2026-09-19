use std::path::PathBuf;
use std::process::Command;

// ==========================================
// CENTRAL CONFIGURATION PARAMETERS
// ==========================================
const REPO_URL: &str = "https://github.com/limine-bootloader/limine.git";
const REPO_BRANCH: &str = "v11.x-binary";

const OS_NAME: &str = "Crusty OS";
const BOOT_TIMEOUT_SECONDS: u32 = 30;
const INTERNAL_KERNEL_NAME: &str = "crusty-core";

const KERNEL_TARGET_JSON: &str = "x86_64-unknown-none";

// Exact Limine release file naming conversions for v11
const LIMINE_BIOS_CD: &str = "limine-bios-cd.bin";
const LIMINE_UEFI_CD: &str = "limine-uefi-cd.bin";
const LIMINE_SYS: &str = "limine-bios.sys";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let should_emulate = args.contains(&"run".to_string());

    // Anchor directly onto the builder's Cargo.toml directory
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Resolve back up to the workspace root directory
    let workspace_root = manifest_dir
        .parent()
        .expect("Failed to find workspace root");

    // Explicitly target the correct project root directory
    let project_root = workspace_root
        .parent()
        .expect("Failed to find project root");

    // Map out paths relative to the workspace layout
    let target_dir = workspace_root.join("target");
    let cache_path = target_dir.join("limine_binaries");
    let staging_path = target_dir.join("iso_staging");
    let kernel_elf = target_dir.join("x86_64-unknown-none/release/kernel");

    // Output the final ISO to the project root
    let iso_output_path = project_root.join("crusty.iso");

    println!("Step 1: Compiling the OS Kernel...");
    let status = Command::new("cargo")
        .current_dir(workspace_root)
        .env(
            "RUSTFLAGS",
            "-C link-arg=-Tkernel/linker.ld -C relocation-model=static -C code-model=kernel",
        )
        .args([
            "build",
            "-p",
            "kernel",
            "--target",
            KERNEL_TARGET_JSON,
            "--release",
        ])
        .status()
        .expect("Failed to execute cargo build");

    if !status.success() {
        std::process::exit(1);
    }

    if staging_path.exists() {
        std::fs::remove_dir_all(&staging_path)
            .expect("Failed to clean staging directory! Close QEMU first.");
    }

    println!("Step 2: Resolving Limine codebase infrastructure...");
    limine_bootloader::clone_bootloader_repository(REPO_URL, REPO_BRANCH, &cache_path).unwrap();
    limine_bootloader::compile_host_tools(&cache_path).unwrap();

    println!("Step 3: Creating boot configuration profile...");
    // Create the boot directories inside the ISO layout
    let limine_data_dir = staging_path.join("boot").join("limine");
    std::fs::create_dir_all(&limine_data_dir).expect("Failed to create boot/limine directory");

    let uefi_boot_dir = staging_path.join("EFI").join("BOOT");
    std::fs::create_dir_all(&uefi_boot_dir).expect("Failed to create UEFI directory structure");

    // LIMINE v11 CONFIG SYNTAX:
    // 1. Lowercase keys (timeout: X instead of TIMEOUT=X)
    // 2. Entries start with '/' instead of ':'
    // 3. 'path:' uses 'boot():/path' instead of 'KERNEL_PATH=boot:///path'
    let conf_text = format!(
        "timeout: {}\n\n/{}\n    protocol: limine\n    path: boot():/boot/{}\n",
        BOOT_TIMEOUT_SECONDS, OS_NAME, INTERNAL_KERNEL_NAME
    );

    // Write limine.conf to root AND /boot/limine to guarantee detection by all boot modes
    limine_bootloader::write_configuration_file(&staging_path.join("limine.conf"), &conf_text)
        .expect("Failed to write Limine config to root");
    limine_bootloader::write_configuration_file(&limine_data_dir.join("limine.conf"), &conf_text)
        .expect("Failed to write Limine config to /boot/limine");

    println!("Step 4: Moving boot binary file assets to staging disk...");
    // 1. Core Kernel Image placement
    limine_bootloader::stage_file(
        &kernel_elf,
        &staging_path.join("boot").join(INTERNAL_KERNEL_NAME),
    )
    .unwrap();

    // 2. Relocate all BIOS components
    limine_bootloader::stage_file(
        &cache_path.join(LIMINE_BIOS_CD),
        &limine_data_dir.join(LIMINE_BIOS_CD),
    )
    .unwrap();
    limine_bootloader::stage_file(
        &cache_path.join(LIMINE_SYS),
        &limine_data_dir.join(LIMINE_SYS),
    )
    .unwrap();

    // 3. Stage UEFI CD binaries
    limine_bootloader::stage_file(
        &cache_path.join(LIMINE_UEFI_CD),
        &limine_data_dir.join(LIMINE_UEFI_CD),
    )
    .unwrap();

    // 4. Stage EFI Executables
    limine_bootloader::stage_file(
        &cache_path.join("BOOTX64.EFI"),
        &uefi_boot_dir.join("BOOTX64.EFI"),
    )
    .unwrap();
    limine_bootloader::stage_file(
        &cache_path.join("BOOTIA32.EFI"),
        &uefi_boot_dir.join("BOOTIA32.EFI"),
    )
    .unwrap();

    println!("Step 5: Stitching final ISO layout and installing boot records...");
    let relative_bios_cd_string = format!("boot/limine/{}", LIMINE_BIOS_CD);
    let relative_uefi_cd_string = format!("boot/limine/{}", LIMINE_UEFI_CD);

    limine_bootloader::assemble_iso_image(
        &staging_path,
        &relative_bios_cd_string,
        &relative_uefi_cd_string,
        &iso_output_path,
    )
    .unwrap();

    limine_bootloader::install_bios_boot_record(&cache_path.join("limine"), &iso_output_path)
        .unwrap();

    println!(
        "Success! Output boot image ready at: {}",
        iso_output_path.display()
    );

    if should_emulate {
        println!("Step 6: Launching system image in QEMU...");
        let qemu_status = Command::new("qemu-system-x86_64")
            .args([
                "-cdrom",
                iso_output_path.to_str().unwrap(),
                "-m",
                "256M",
                "-M",
                "q35",
                "-serial",
                "stdio",
                "-display",
                "gtk",
                "-no-reboot",
            ])
            .status()
            .expect("Failed to execute QEMU emulator");

        if !qemu_status.success() {
            std::process::exit(qemu_status.code().unwrap_or(1));
        }
    }
}

