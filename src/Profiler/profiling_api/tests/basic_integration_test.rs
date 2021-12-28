use io::Write;
use std::io;
use std::sync::Once;
mod common;

static INIT: Once = Once::new();

fn initialize() {
    INIT.call_once(|| {
        common::build_test_profiler("basic_integration_test");
        common::build_dotnet_core_project("HelloWorld");
    });
}

#[test]
fn given_something_when_something_then_something() {
    initialize();
    let output = common::run_dotnet_core_profiled_process(
        "HelloWorld",
        "3.0",
        "DF63A541-5A33-4611-8829-F4E495985EE3",
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    print!("{}", stdout);
    io::stdout().flush().unwrap();
}
