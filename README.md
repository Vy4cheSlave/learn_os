All source code is based on the article:
```bash
https://github.com/phil-opp/blog_os.git
```

# learn_os
I'm learning the basics of system programming, starting with how to write an OS

To create a binary file, you need to enter the command in the project directory:
```bash
rustup override set nightly
```

And also create a config.yml file in the .cargo directory and add the following lines:
```bash
# in .cargo/config.toml

[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]
```

Then enter the following in the terminal:
```bash
rustup component add rust-src
```

Now it is possible to build a binary file with the following command:
```bash
cargo build --target x86_64-learn_os.json
```
Or if you want full performance from a binary file, then:
```bash
cargo build --release --target x86_64-learn_os.json
```