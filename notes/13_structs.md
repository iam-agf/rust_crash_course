
# Rust crash course

### Struct

There are two methods to create a structure:

* Defining the content the traditional way
* Creating it with a tuple

```rust
struct Color(i8, i8, i8)

struct Person{
    first_name: String,
    last_name: String,
}
```
 
If we require to run a function that will depend on the structure we defined we can use `impl`

```rust
impl Person{
    fn name(self)-> String{
        format!("{} {}", self.first_name, self.last_name)
    }
}
```
