## Text on the screen?
For basic output (at least in the beginning), this project will use the Video Graphics Array (VGA) standard.
VGA text mode is a part of the VGA Standard introduced by IBM in 1987.
This standard provides a text buffer as a 2-D array that can be filled with 2 byte chunks of data such that each chunk is interpreted as follows:


| Bit number | Interpretation   |
| ---------- | ---------------- |
| 0 - 7      | ASCII Code       |
| 8 - 11     | Foreground color |
| 12 - 14    | Background color |
| 15         | Blink            |


### Implementing basic Output
- The encoding and writing of characters is handled by `vga_buffer::Writer` which holds position information as well as `vga_buffer::ColorCodes` and can build and display `vga_buffer::ScreenChars` from any type which implements `AsRef<str>`.

- The [VGA Function Specification](https://web.archive.org/web/20150816220334/http://www.eyetap.org/cyborgs/manuals/soft_vga.pdf) notes that the VGA buffer is located at `0xb8000` and through `unsafe` Rust code we can initialize `Writer::buffer` with this raw memory address.

- We then define a global `static` variable `WRITER` that wraps a `Writer` in a spin-lock mutex for interior mutability.

- In addition to implementing `fmt::Writer` for `Writer`, some macros that emulate `println!()` and `print!()` are defined for easier printing and debugging.

- In the case of panics, the panic handler simple prints out the `PanicInfo` via our new `println!()` macros.

> [!NOTE] 
> In addition to this overview, I have made an effort to write extensive comments that describe not only the what and how, but the why!
