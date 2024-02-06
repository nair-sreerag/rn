# Rust commands

1. To add a crate to the project

   > cargo add **<package name\>** or add the crate with its version in the **Cargo.toml** file under the dependencies section.

2. To run the project

   > cargo run

3. To watch for file changes

   > cargo watch -- cargo run

   This command watches for any file changes and runs the command **cargo run** if any change is observed

4. To generate documentation for your code
   > cargo doc --open
