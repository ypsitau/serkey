# A simple keycode parser for handling UTF-8 and VT100 terminal ctrl sequences

This library provides a simple keycode parser that reads VT100 terminal control sequences from an input source and converts them into key events. The input source can be any type that implements the `embedded_io::Read` or `embedded_io_async::Read` traits.

Create a new parser instance using `serkey::Reader::new()`.

```rust
let mut serkey_reader = serkey::Reader::new();
```

The associated functions `serkey::Reader::read_key` and `serkey::Reader::read_key_blocking` read a key from the given reader asynchronously and synchronously, respectively, and return it as an `Option<serkey::Key>`.

`serkey::Key` has three types of variants:

- `CookedChar(char)`: Represents a regular character key.
- `CookedCtrl(u8)`: Represents a control key combination (e.g., Ctrl-A).
- Other variants represent individual key code like arrow keys, delete, etc. For instance, `serkey::Key::Left(attr)` represents the left arrow key and `serkey::Key::Delete(attr)` represents the delete key, where `attr` contains additional attributes like modifier keys (Shift, Ctrl, Alt).

## Examples

```rust
async fn print_key_async(mut reader: impl embedded_io_async::Read) {
    let mut serkey_reader = serkey::Reader::new();
    loop {
        if let Some(key) = serkey_reader.read_key(&mut reader).await {
            match key {
                serkey::Key::CookedChar('a') => println!("'a' is pressed"),
                serkey::Key::CookedCtrl(0x01) => println!("Ctrl-A is pressed"),
                serkey::Key::CookedCtrl(0x03) => break,
                serkey::Key::Left(attr) => println!("[Left] is pressed with {:?}", attr),
                serkey::Key::Delete(attr) => println!("[Delete] is pressed with {:?}", attr),
                _ => println!("{}", key),
            }
        }
    }
}
```

```rust
fn print_key_blocking(mut reader: impl embedded_io::Read) {
    let mut serkey_reader = serkey::Reader::new();
    loop {
        if let Some(key) = serkey_reader.read_key_blocking(&mut reader) {
            match key {
                serkey::Key::CookedChar('a') => println!("'a' is pressed"),
                serkey::Key::CookedCtrl(0x01) => println!("Ctrl-A is pressed"),
                serkey::Key::CookedCtrl(0x03) => break,
                serkey::Key::Left(attr) => println!("[Left] is pressed with {:?}", attr),
                serkey::Key::Delete(attr) => println!("[Delete] is pressed with {:?}", attr),
                _ => println!("{}", key),
            }
        }
    }
}
