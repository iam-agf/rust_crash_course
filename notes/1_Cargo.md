
# Rust crash course

### Cargo 
If you want to start your project, usually you run `cargo init`. Nonetheless this is not strictly necessary since you can run the project just by doing `rustc filename.rs` generating this way the executable `filename`for the file. 

Considering you start the project using `cargo` (ie talking on the terminal), you can start with `cargo new folder_name` where `folder_name` is the name of the folder where your project will start. If you want to start your project in the current folder, you can do `cargo init`. This will generate

```
📦folder_name
 ┣ 📂src
 ┃ ┗ 📜main.rs
 ┣ 📜 .gitignore
 ┗ 📜  Cargo.toml
```

where `main.rs` will be the part where your code will be ran. Now, you can
- Run your project: `cargo run`
- Compile/Build your project not for production: `cargo build`
- Compile/Build your project for production: `cargo build --release`


In the case you make `cargo run`, you will get the `debug` folder 
```
📂target
 ┣ 📂src
 ┃ ┗ 📜main.rs
 ┣ 📜 .gitignore
 ┗ 📜  Cargo.toml
```