A 8 bit cpu emulator i built in rust.

Here are some features to note:

- Custom instruction set
- 16 bit addess (64k x 8 memory, 64k x 8 flash)
- 0-32k is user space memory, 32-64k is display buffer.

To assemble install `customasm`.

```sh
cargo install customasm
customasm examples/fibonacci.asm -o prog.bin
```

Run

```sh
cargo run -- prog.bin
```