
# Rust crash course

### Strings

There are two types of strings:
 - Immutable (fixed-length)
 - Growable (heap-allocated data structure)

If you define your variable as 

```rust
let text = "this";
```

You will define an immutable string. In the case you want a growable string, you require to write

```rust
let mut text = String::from("this");
```

### Some functions

* `text.len()`: Size of `text` (string) variable.
* `text.push(c)`: Adds one character `c` to the (string) variable `text`
* `text.push_str(word)`: Adds a string `word` to the (string) variable `text`
* `text.capacity()`: Checks bytes capacity of `text` (string) variable. 
* `text.is_empty()`: Boolean value that is true if `text` (string) variable is empty else is false. 
* `text.capacity`: Checks bytes capacity of `text` (string) varibale. 
* `text.contains`: Boolean that verifies if a string is inside of `text` (string) varibale. 
* `text.replace('word1', 'word2')`: Replaces **all** the substrings `word1` with `word2` from the `text` (string) varibale. 
* `text.split_whitespace()`: Splits the `text` (string) variable through the whitespaces.


