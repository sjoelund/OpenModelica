use openmodelica_backend::Main;
use std::env::args;

fn main() -> () {
    Main::main(args().collect())
}
