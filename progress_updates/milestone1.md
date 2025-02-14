# RustOS

## A Freestanding Binary
- The first thing we need to do is remove our dependency on the standard library
	- This is because many of the standard library functions result in system calls and we need to implement those system calls ourselves!
- This is done with the attribute `#![no_std]` 
- Additionally, when a Rust program panics it unwinds the stack to ensure that all variables are freed and the running program can catch and handle the panic. 
	- This requires some specific libraries, `libunwind` on Linux, that introduce a lot of complexity
	- We can simply abort on panics with 

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

### Maybe Building

<details>
    <summary>Attempting to Build</summary>
 ➜ cargo build
    Compiling os v0.1.0 (/home/sam/dev/rust/os)
 error: linking with `cc` failed: exit status: 1
   |
   = note: LC_ALL="C" PATH="/home/sam/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/sam/.ghcup/bin:/home/sam/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/snap/bin:/home/sam/.dotnet/tools:/home/sam/.cabal/bin" VSLANG="1033" "cc" "-m64" "/tmp/rustcRk4hyJ/symbols.o" "/home/sam/dev/rust/os/target/debug/deps/os-33e596e7879cd5bb.2bk3rxgvnet7w1ne.rcgu.o" "-Wl,--as-needed" "-L" "/home/sam/dev/rust/os/target/debug/deps" "-L" "/home/sam/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/sam/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-f37052492751c579.rlib" "/home/sam/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fd15ec7f305d48e7.rlib" "/home/sam/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-d700583125da6701.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/home/sam/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/home/sam/dev/rust/os/target/debug/deps/os-33e596e7879cd5bb" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"   = note: /usr/bin/ld: /home/sam/dev/rust/os/target/debug/deps/os-33e596e7879cd5bb.2bk3rxgvnet7w1ne.rcgu.o: in function `_start':            /home/sam/dev/rust/os/src/main.rs:13: multiple definition of `_start'; /usr/lib/gcc/x86_64-linux-gnu/13/../../../x86_64-linux-gnu/Scrt1.o:(.text+0x0): first defined here  /usr/bin/ld: /usr/lib/gcc/x86_64-linux-gnu/13/../../../x86_64-linux-gnu/Scrt1.o: in function `_start':  (.text+0x1b): undefined reference to `main'
           /usr/bin/ld: (.text+0x21): undefined reference to `__libc_start_main'
           collect2: error: ld returned 1 exit status
 
   = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
 
 error: could not compile `os` (bin "os") due to 1 previous error

</details>

- We can fix this linker error one of two ways:
	1. by building for an **embedded system** that has *no operating system*
	2. passing special flags to the linker

#### Fixing Linking (for Linux systems)
 - The linker includes the C runtime (`_start`, which we have no overidden).
 - Obviously this is an issue, but we can tell `rustc` to **not** include these with
 ```
 cargo rustc -- -C link-arg=-nostartfiles
 ```
#### Building for **METAL**
- After defining our own entry point let's try and build for a bare metal target
- We can download this build target with 

```bash
rustup target add thumbv7em-none-eabihf
```

- And build with
```bash
cargo build --target thumbv7em-none-eabihf
```

- This is a just an example bare metal target, we can define our own custom build target as seen below


## The Boot Process
- The boot process for a computer moves through these (generalized) steps:
	1. Firmware code located on the *Motherboard ROM* begins executing by performing a *power-on self test* or **POST**
	2. This process, the *BIOS*,  detects available RAM,  pre-initializes the CPU and hardware,  and looks for the bootable OS image on disk
	3. If a bootable disk is found, control is transferred to the *bootloader* which is 512-byte portion of executable code stored at the beginning of the disk.
		- Since many boot loaders are actually larger than 512-bytes, a preliminary "first" bootloader is placed in this region which loads the primary "actual" bootloader
	4. The bootloader determines the location of the kernel image and loads it into memory. The bootloader also:
		- Changes the CPU from 16-bit *real-mode* to 32-bit *protected* or *kernel* mode
		- Queries information from the BIOS (like memory maps) and passes it to the kernel


### Building for our custom target

- When `cargo` builds an executable, the format of that executable is based on the *target triple*

```json
{
    "llvm-target": "x86_64-unknown-none", 
    "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": "32",
    "os": "none",
    "executables": true,
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",
    "panic-strategy": "abort",
    "disable-redzone": true,
    "features": "-mmx,-sse,+soft-float"
}
```

- Walking though these options:

- `llvm-target` - This is the *target-triple* as defined [here](https://clang.llvm.org/docs/CrossCompilation.html#target-triple)
- `data-layout` - defines the size of various data types
- `arch` - CPU architecture
- `target-endian` - endianess (which way do you like your egg)
- `target-pointer-width` - pointer width (obv) 
- `target-c-int-width` - int width (obv)
- `os` - here we don't *have* an OS because *we* are the OS
- `executables` - 
- `linker-flavor` - we set the linker to use the cross-platform lld linker that ships with Rust (to ensure that linux targets are supported)
- `linker` - see linker-flavor
- `panic-strategy` - defines that we want to just abort the program on panic instead of unwinding the stack
- `disable-redzone` 
    - the **redzone** is a 128-byte region below the current stackframe that is part of the SystemV ABI.
    - It allows functions to store temporary data in this regions without adjusting the stack pointer
    - Because we aren't going to be doing anything fancy with separate kernel stacks (and instead just push expection frames directly on the stack) any expection would **overwrite** the red-zone!
    - to avoid this entirely, we will just no allow the redzone
- `features` this field allows us to disable or enable any relevant features
    - `-mmx,-sse` disables support for SIMD instructions
        - this makes sense in the context of a kernel:
            - when the kernel takes over execution it stores register state for the prempted program. 
            - Saving large SIMD registers can lead to performance problems


- In addition to building our code for this custom target, we also need to build the core features of Rust to work on this architecture. This can be accomplished by instructing cargo to rebuild `core`, `compiler_builtins`, and `compiler-builtins-mem`.

```toml
# ./cargo/config.toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]
```

- Now our kernel will build!

- To enable our compiled kernel to actually run, the executable needs to contain a bootloader to initialize the CPU and start *kernel mode*

- For this, the `bootloader` crate is being used which creates a bootable disk image in `target/x86_64-rustos/debug/bootimage-os.bin`. 

- To automate the run process, we can add the following to our `.carg/config.toml`

```toml
# ./cargo/config.toml

[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```

