use std::env;
use std::process::{Command, Output};

fn root_prefix<'a>() -> &'a str {
    let pwd = env::current_dir().unwrap();
    if pwd.ends_with("clr-profiler") {
        ""
    } else {
        "../"
    }
}

pub fn build_test_profiler(test_profiler: &str) -> Output {
    let manifest_path = format!("{p}test_profilers/Cargo.toml", p = root_prefix());
    let output = Command::new("cargo")
        .args(&[
            "build",
            "--verbose",
            "--features",
            test_profiler,
            "--manifest-path",
            &manifest_path,
        ])
        .output()
        .expect("Failed to execute profiler build process.");
    if !output.status.success() {
        panic!("Non-zero return code encountered when trying to build test profiler.");
    } else {
        output
    }
}

pub fn build_dotnet_core_project(project_name: &str) -> Output {
    // TODO: This file path will fail if we try to run these tests on windows.
    //       Come up with cross-platform way of building paths.
    let project_path = format!(
        "{p}test_clr/{n}/{n}.csproj",
        p = root_prefix(),
        n = project_name
    );
    let output = Command::new("dotnet")
        .args(&["build", &project_path])
        .output()
        .expect("Failed to execute .NET build process.");
    if !output.status.success() {
        panic!("Non-zero return code encountered when trying to build test .NET project.");
    } else {
        output
    }
}

pub fn run_dotnet_core_profiled_process(
    project_name: &str,
    dotnet_version: &str,
    clsid: &str,
) -> Output {
    // TODO: This file path will fail if we try to run these tests on windows.
    //       Come up with cross-platform way of building paths.
    let dotnet_artifact_path = format!(
        "{p}test_clr/{n}/bin/Debug/netcoreapp{v}/{n}.dll",
        p = root_prefix(),
        n = project_name,
        v = dotnet_version
    );
    let profiler_artifact_path = format!("{p}target/debug/libtest_profilers.so", p = root_prefix());
    let clsid = format!("{{{c}}}", c = clsid);
    let output = Command::new("dotnet")
        .arg(dotnet_artifact_path)
        .env("CORECLR_ENABLE_PROFILING", "1")
        .env("CORECLR_PROFILER", clsid) // TODO: Should this be hard-coded?
        .env("CORECLR_PROFILER_PATH", &profiler_artifact_path)
        .output()
        .expect("Failed to execute .NET process.");
    if !output.status.success() {
        panic!("Non-zero return code encountered when trying to run test .NET project.");
    } else {
        output
    }
}

// pub fn spawn_dotnet_core_profiled_process(project_name: &str, dotnet_version: &str) {
// TODO: For long running processes, need to spawn and return a handle, so things
//       like http servers can have requests made against them and confirm the
//       profiling results. See: https://doc.rust-lang.org/std/process/struct.Child.html
// }
