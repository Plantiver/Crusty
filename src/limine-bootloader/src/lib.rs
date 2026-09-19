use std::fs;
use std::path::Path;
use std::process::Command;

pub fn clone_bootloader_repository(
    repo_url: &str,
    branch: &str,
    destination: &Path,
) -> Result<(), String> {
    if destination.exists() {
        return Ok(());
    }
    let status = Command::new("git")
        .args([
            "clone",
            repo_url,
            destination.to_str().ok_or("Invalid destination")?,
            "--branch",
            branch,
            "--depth",
            "1",
        ])
        .status()
        .map_err(|e| format!("Git failure: {}", e))?;
    if !status.success() {
        return Err("Git clone failed".to_string());
    }
    Ok(())
}

pub fn compile_host_tools(directory: &Path) -> Result<(), String> {
    let status = Command::new("make")
        .current_dir(directory)
        .status()
        .map_err(|e| format!("Make failure: {}", e))?;
    if !status.success() {
        return Err("Make failed".to_string());
    }
    Ok(())
}

pub fn write_configuration_file(file_path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(file_path, content).map_err(|e| format!("File write error: {}", e))?;
    Ok(())
}

pub fn stage_file(source: &Path, destination: &Path) -> Result<(), String> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::copy(source, destination).map_err(|e| format!("Copy failed: {}", e))?;
    Ok(())
}

pub fn assemble_iso_image(
    staging_dir: &std::path::Path,
    bios_bin_name: &str,
    uefi_bin_name: &str,
    output_iso: &std::path::Path,
) -> Result<(), String> {
    // Prevent xorriso silent failures if the file is locked by a zombie QEMU process
    if output_iso.exists() {
        let _ = std::fs::remove_file(output_iso);
    }

    let status = std::process::Command::new("xorriso")
        .args([
            "-as",
            "mkisofs",
            "-r", // Normalize file permissions (CRITICAL)
            "-J", // Add Joliet for fallback filename parsing
            "-b",
            bios_bin_name,
            "-no-emul-boot",
            "-boot-load-size",
            "4",
            "-boot-info-table",
            "--efi-boot",
            uefi_bin_name,
            "-efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            "-o",
            output_iso.to_str().unwrap(),
            staging_dir.to_str().unwrap(),
        ])
        .status()
        .map_err(|e| format!("Xorriso failed: {}", e))?;

    if !status.success() {
        return Err("Xorriso returned non-zero status".to_string());
    }
    Ok(())
}

pub fn install_bios_boot_record(limine_utility: &Path, target_iso: &Path) -> Result<(), String> {
    let status = Command::new(limine_utility)
        .args([
            "bios-install",
            target_iso.to_str().ok_or("Invalid target path")?,
        ])
        .status()
        .map_err(|e| format!("Limine install utility failed: {}", e))?;
    if !status.success() {
        return Err("Limine MBR update failed".to_string());
    }
    Ok(())
}
