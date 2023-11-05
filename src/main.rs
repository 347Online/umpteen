use umpteen::{report, Runtime};

fn main() {
    let mut vm = Runtime::new();
    vm.sample_program().unwrap();
}
