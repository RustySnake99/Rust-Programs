# Rust-Programs
A Rust project that contains my rust programs that I made to learn the various concepts of the language. Rust is a general purpose programming language that prioritizes performance, speed, type safety and concurrency. This project assumes that the user has rust and cargo installed via `rustup` along with Visual Studio Build Tools.
To add more binaries (additional rust programs) into the same rust project, simply create a folder `bin` within the `src` directory and store them in this. Ex: `src/bin/addition.rs`

## Important Ruat Commands
These are aome important rust commands that have to be used while starting any rust project. Firstly, to start any new rust project, run the following command: `bash cargo new <project-name>`.
Then, to build and run the project, use the command: `cargo run`. Other commands are as follows:

`cargo build`: Compiles the current Rust project. Additionally, use the --release flag for a more concise and deployment oriented compilation of the project.
`cargo test`: Run tests and test cases for the current project.
`cargo run --bin <file_name>`: Run a specific binary from the project. It is to be noted that the file name mentioned along with this command should be without any extensions.
`cargo add <crate_name>`: Add a particular crate to the project.
`cargo update`: Updates all the dependencies mentioned within the `cargo.toml` file.

An example of a Rust program is:
```
fn main() {
  pritnln!("Hello World!");
}
```
