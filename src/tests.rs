#[cfg(test)]
#[test]
fn test_kernel() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    // choose whether to start the UEFI or BIOS image
    let uefi = true;

    let mut cmd = std::process::Command::new(r#"C:\Program Files\qemu\qemu-system-x86_64"#);
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!(r#"format=raw,file={uefi_path}"#));
    } else {
        cmd.arg("-drive")
            .arg(format!(r#"format=raw,file={bios_path}"#));
    }
    println!("{:?}", cmd);
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();

    // Ваш код для запуска ядра с аргументом --test в QEMU
    // Например:
    // let output = std::process::Command::new("qemu-system-x86_64")
    //     .arg("-kernel")
    //     .arg("path/to/your/kernel")
    //     .arg("--test")
    //     .output()
    //     .expect("Failed to execute QEMU command");

    // Проверка кода возврата
    // assert!(output.status.success(), "Kernel tests failed: {:?}", output);
}
