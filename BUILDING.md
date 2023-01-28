# Building

## Prerequisites

- .NET SDK 7.0
- Rust toolchain
- *Recommended OS*: Any desktop platform you'd like (Windows / Linux / MacOS ARM64 or Intel)
- *Recommended IDE*: Any of Visual Studio, Visual Studio Code or JetBrains Rider. Make sur you have both C# and Rust LSP for a comfortable experience, with everything in a single IDE.

## Building

As Dr-Dotnet is an hybrid C# / Rust project, it makes little sense to build the C# and Rust parts separately.
The recommended workflow is the following:
- Clone the repository
- Open `src/DrDotnet.sln` in your IDE
- Build either `DrDotnet.Web.csproj`, `DrDotnet.Desktop.csproj`, or the solution `DrDotnet.sln`, depending on how you plan to use Dr-Dotnet.

The `DrDotnet.csproj` project has a prebuild step that will try to build the Rust profilers. If it fails, you'll find the Rust compiler output in the Output window for instance if you are using Visual Studio. You usually don't need to use cargo commands yourself directly at this first stage.

You can also simply use dotnet build or dotnet run if you are more a CLI person 😊

## Creating new profilers

The `DrDotnet.sln` links to the Rust part which is ann in the `src/DrDotnet.Profilers` directory.   
The Rust codebase is divider in two projects:
- `profilers` is where all profilers are
- `profiling_api` is where is CLR profiling API bindings are (it wraps the unsafe pointed based C syntax and brings some safety and convenience to it)

To create a new profiler, checkout `src/DrDotnet.Profilers/profilers/src/profilers/` and checkout any profiler. You can basically duplicate one and change its UUID (make it unique). Then, head to `src/DrDotnet.Profilers/profilers/src/lib.rs/` and add the new profiler in the `register!` macro. Then you're good to go, you can now start using the CLR Profiling API. Checkout [the official documentation to get started](https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/profiling-interfaces).
I also recommend checking our [Christophe Nasarre blog posts](https://chnasarre.medium.com/start-a-journey-into-the-net-profiling-apis-40c76e2e36cc) for a more "friendly" introduction to this API ;)    

Note: You'll need to set `is_released` in your profiler to true if you want to be able to view your profiler in the C# UI when built in release mode.    
Another note: DrDotnet attaches to an already running process, meaning that not all callbacks are usable, only those who can be enable after attaching. See the flags [here](https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/cor-prf-monitor-enumeration) and [here](https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/cor-prf-high-monitor-enumeration).

## Useful Links

If you take on an adventure to develop / improve profilers, here are few useful links to get into the swing of things:

- [Pavel Yosifovich — Writing a .NET Core cross platform profiler in an hour](https://www.youtube.com/watch?v=TqS4OEWn6hQ)
- [Pavel Yosifovich — DotNext Moscou 2019 Source Code](https://github.com/zodiacon/DotNextMoscow2019)
- [Josef Biehler - Create a .NET profiler with the Profiling API](https://dev.to/gabbersepp/create-a-net-profiler-with-the-profiling-api-start-of-an-unexpected-journey-198n)
- [Mattias Högström - Building a Mixed-Mode Sampling Profiler](https://www.codeproject.com/Articles/384514/Building-a-Mixed-Mode-Sampling-Profiler)
- [Christophe Nasarre - Start a journey into the .NET Profiling APIs](https://chnasarre.medium.com/start-a-journey-into-the-net-profiling-apis-40c76e2e36cc)
- [Some random COM C++ source code](https://github.com/tenable/poc/blob/master/Comodo/Comodo%20Antivirus/ComodoInjectionCode/ComodoInjectionCode/InjectedCode.cpp)
- [Some random COM C++ source code](https://cpp.hotexamples.com/examples/-/ICLRRuntimeInfo/GetInterface/cpp-iclrruntimeinfo-getinterface-method-examples.html)

