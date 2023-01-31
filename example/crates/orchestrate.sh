rustc --crate-type=lib rary.rs

rustc executable.rs --extern rary=library.rlib
./executable

# chmod +x orchestrate.sh