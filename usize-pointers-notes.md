# `usize`, pointers, and vector indexing

## Core idea

- `u8` is a byte value (`0..=255`).
- `usize` is a machine-sized unsigned integer:
  - 32 bits on 32-bit targets
  - 64 bits on 64-bit targets
- Rust uses `usize` for memory-related quantities like:
  - lengths (`Vec::len`)
  - capacities (`Vec::with_capacity`)
  - indexes (`v[i]`)
  - offsets in pointer math

## Why not `u8` for vector length?

`Vec` length/index/capacity are memory-size concepts. They must be able to represent platform-sized memory counts, so Rust uses `usize`.

If length were `u8`, it would cap at 255 and would not match the standard indexing/allocation APIs.

## Mental model

- A pointer stores an address.
- Index `i` means "start of element `i`."
- Address of element `i` is:

```text
base_address + i * size_of::<T>()
```

So moving by one element may move multiple bytes, depending on `T`.

## Example

```rust
use std::mem::size_of;

fn main() {
    let v: Vec<u32> = vec![10, 20, 30, 40];
    let base = v.as_ptr(); // *const u32

    let i: usize = 2;
    let elem_ptr = unsafe { base.add(i) }; // moves i * size_of::<u32>() bytes

    println!("size_of::<u32>() = {}", size_of::<u32>()); // 4
    println!("v[{i}] = {}", unsafe { *elem_ptr }); // 30
}
```

In this example, `add(2)` moves `2 * 4 = 8` bytes from the base pointer because each `u32` is 4 bytes.
