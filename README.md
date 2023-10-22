All source code is based on the article:
### ! The course is not relevant in places. !
```bash
https://github.com/phil-opp/blog_os.git
```

# learn_os
I'm learning the basics of system programming, starting with how to write an OS

To run the execution, the Rust compiler must be installed on your computer.

In the directory convenient for you in the terminal, enter the following command:
```bash
git clone https://github.com/Vy4cheSlave/learn_os.git
```
## Attention! 
When cloning a repository, rust-analyzer issues configuration errors. I don't know yet if this leads to any problems.

To create a binary file, you need to enter the command in the project directory:
```bash
# in learn_os/kernel

rustup override set nightly
rustup component add llvm-tools-preview

# in learn_os
rustup override set nightly
```

And also create a config.toml file in the .cargo directory and add the following lines:
```bash
# in .cargo/config.toml

[unstable]
# enable the unstable artifact-dependencies feature, see
# https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
bindeps = true
```
## Attention!
For further work, you need to install QEMU on your device (https://www.qemu.org/).
```rust
// in learn_os/src/main.rs
// replace with your own QEMU location

let out cmd = std::process::Command::new(r#"full\path\to\qemu\qemu-system-x86_64.exe "#);

// and also
// choose whether to start the UEFI or BIOS image
let uefi = true;
// or
let uefi = false;
```

In case you want to put it on your device or virtual machine, you can find out the location of .img files by replacing everything in (#in learn_os/src/main.rs) with the following:
```rust
fn main() {
    println!("{}", env!("UEFI_PATH"));
}

// or

fn main() {
    println!("{}", env!("BIOS_PATH"));
}
```

The final step for these options is the same.
```bash
cargo build
cargo run
```
