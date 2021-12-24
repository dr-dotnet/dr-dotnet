# Traceman

🚧 Very WIP 😉

The goal is to create a profiler that would ease the tracking of common issues in .NET applications suchs as deadlocks, cpu hotpaths, zombie threads, async hotpaths (stuck tasks), memory leaks.  
It is possible to identify some of these issues using today's existing tools, but with some drawbacks:

- Often you have to explicitely look for such issue.
- Such tools rarely have a good user experience. Appart from dotMemory, I find UX considerations to be often left appart.
- In many cases the tracing/profiling has a noticable impact on the runtime and add bias to the profiling results.

The .NET Profiling API is accessible via COM interop (cross-platform thanks to the Platform Adaptation Layer) and allows little overheap profiling compared to other methods while giving a wide range of possibilities for profiling purpose. Perfview uses this API (on top of ETW) however it does it from managed code calling mocked COM objects written in C#. For the project, the idea would be to do it in C++ or even better in Rust (ideally Rust + including CoreCLR headers in kind of a hybrid fashion, if that is possible). Then the offline tooling (UI) can be in C# or any other "productive" language.

## Useful Links

- [Pavel Yosifovich — Writing a .NET Core cross platform profiler in an hour](https://www.youtube.com/watch?v=TqS4OEWn6hQ)
- [Pavel Yosifovich — DotNext Moscou 2019 Source Code](https://github.com/zodiacon/DotNextMoscow2019)
- [Josef Biehler - Create a .NET profiler with the Profiling API](https://dev.to/gabbersepp/create-a-net-profiler-with-the-profiling-api-start-of-an-unexpected-journey-198n)
- [Mattias Högström - Building a Mixed-Mode Sampling Profiler](https://www.codeproject.com/Articles/384514/Building-a-Mixed-Mode-Sampling-Profiler)
- [Christophe Nasarre - Start a journey into the .NET Profiling APIs](https://chnasarre.medium.com/start-a-journey-into-the-net-profiling-apis-40c76e2e36cc)
- [Some random COM C++ source code](https://github.com/tenable/poc/blob/master/Comodo/Comodo%20Antivirus/ComodoInjectionCode/ComodoInjectionCode/InjectedCode.cpp)
- [Some random COM C++ source code](https://cpp.hotexamples.com/examples/-/ICLRRuntimeInfo/GetInterface/cpp-iclrruntimeinfo-getinterface-method-examples.html)
