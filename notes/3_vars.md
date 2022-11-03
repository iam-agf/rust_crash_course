
# Rust crash course

### Variables

You define a variable by calling 

`let variable_name = 'content'`

If you try to update a variable, Rust will return an error because you need to declare variables as 'mutable' if you want to update them during your process adding `mut` to the variable:

```rust
// This way is valid
let mut variable = 37;
variable = 38;
```

Like in Javascript or other languages, we have also a `const` keyword, that makes the variable inmutable (you can't update it nor change it)

`const constant = "I can't update this"`

Finally, you can define several variables at the same time by doing

`let (my_name, my_age) = ("Chad", 29)`