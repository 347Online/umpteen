use umpteen::Runtime;

fn main() {
    let mut vm = Runtime::new();
    vm.load_source(r#"print "Hello World""#);
    vm.run()
}
