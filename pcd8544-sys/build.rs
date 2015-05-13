
extern crate gcc;

fn main() {
    gcc::compile_library("libpcd8544.a", &["RaspberryPi/libraries/c/PCD8544/PCD8544.c"]);
}
