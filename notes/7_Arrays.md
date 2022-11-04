
# Rust crash course

### Arrays

In the length of arrays, they have a fixed one, not like in other languages. 

Defining the array with it content and not filling the size will return an error.

To get an element specific of the array just write

```rust
let numbers: [i32;5] = [1,2,3,4,5];
println!("{}", numbers[0])
```

You can get slices of arrays with


```rust
let numbers: [i32;5] = [1,2,3,4,5];
let slice: &[i32] = &numbers[1..3];
```rust
