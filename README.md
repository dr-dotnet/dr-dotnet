# Dr-Dotnet 🩺

🚧 Very WIP 😉

The goal is to create a profiler that would ease the tracking of common issues in .NET applications such as deadlocks, cpu hotpaths, zombie threads, async hotpaths (stuck tasks), memory leaks.  
It is possible to identify some of these issues using today's existing tools, but with some drawbacks:

- Often you have to explicitely look for such issue.
- Such tools rarely have a good user experience. Appart from dotMemory, I find UX considerations to be often left appart.
- In many cases the tracing/profiling has a noticable impact on the runtime and add bias to the profiling results.

The .NET Profiling API is accessible via COM interop (cross-platform thanks to the Platform Adaptation Layer) and allows little overheap profiling compared to other methods while giving a wide range of possibilities for profiling purpose. Perfview uses this API (on top of ETW) however it does it from managed code calling mocked COM objects written in C#. For the project, the idea would be to do it in C++ or even better in Rust (ideally Rust + including CoreCLR headers in kind of a hybrid fashion, if that is possible). Then the offline tooling (UI) can be in C# or any other "productive" language.

## Todo

- [x] Manage to initialize at start
- [x] Manage to initialize on attach
- [x] Write profiler POC in rust
- [x] Add a functional build flow (profiler + attacher)
- [x] Figure out what I really want feature-wise
  - Should I even consider attach mode? (because of its limitations)
  - ~~Should I even consider start mode? (because it is too invasive and implies process restart, which is bad is issue is live)~~
- [ ] ~~Implement IPC through Event Pipe~~
- [x] Add simple UI client for attaching
- [ ] ~~Add simple UI client for setting up on-start profiler~~
- [x] Create web app profiler UI that can work in headless scenarios (eg. linux servers)
- [ ] Have web app profiler work when used as a sidecar docker container
- [x] Use **ffidji** to have an interface to share profiler specifications between profilers lib and UI
  - For each profiler: Name, Guid, Description, Duration, Parameters (blob?)
- [x] Find out analysis format
  - What kind of data should it hold? Summary of analysis or should it allow some kind of browsing?
  - json? yaml? custom?
- [x] Implement wrapper around CLR types API
- [ ] Add utilitary function to get stack traces (possible to get line n° ?)
- [ ] Finish the exception profiler (add stack traces, ordering by importance, duration, ...)
- [ ] Choose next profiler
  - Memory leak detector with suriviving references?
  - High allocations (ObjectsAllocatedByClass)?

## Possibilities

- On object allocated with [ObjectAllocated](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback-objectallocated-method) `COR_PRF_MONITOR_OBJECT_ALLOCATED` ⚠️
- Object reference is moved during garbage collection with [MovedReferences](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback-movedreferences-method) `COR_PRF_MONITOR_GC`
- Build reference tree after a GC run with [ObjectReferences](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback-objectreferences-method) `COR_PRF_MONITOR_GC`
- Object instance allocation per class with [ObjectsAllocatedByClass](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback-objectsallocatedbyclass-method) `COR_PRF_MONITOR_GC`
- Unmanaged code called from managed code `COR_PRF_MONITOR_CODE_TRANSITIONS` ⚠️
- Get surviving references after garbage collection `COR_PRF_MONITOR_GC` [SurvivingReferences2](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method)
- On runtime suspended / resumed `COR_PRF_MONITOR_SUSPENDS` [RuntimeSuspendStarted](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback-runtimesuspendstarted-method)
- On garbage collection started / finished `COR_PRF_MONITOR_GC`
- On thread created `COR_PRF_MONITOR_THREADS`
- On exception thrown `COR_PRF_MONITOR_EXCEPTIONS`
- On method enter / leave with [SetEnterLeaveFunctionHooks3WithInfo](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo3-setenterleavefunctionhooks3withinfo-method?WT.mc_id=DT-MVP-5003325) `COR_PRF_MONITOR_ENTERLEAVE` ⚠️
- Write to event pipe
- Request stacktrace (per thread) `COR_PRF_ENABLE_STACK_SNAPSHOT`
- Annnnd a lot more :p... `COR_PRF_MONITOR_GC`

### Profiler is initialized on application start

Any flag from [COR_PRF_MONITOR enumeration](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/cor-prf-monitor-enumeration).

### Profiler is initialized on attach

Only possible on a subset of [COR_PRF_MONITOR enumeration](https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/cor-prf-monitor-enumeration):

- `COR_PRF_MONITOR_THREADS`
- `COR_PRF_MONITOR_MODULE_LOADS`
- `COR_PRF_MONITOR_ASSEMBLY_LOADS`
- `COR_PRF_MONITOR_APPDOMAIN_LOADS`
- `COR_PRF_ENABLE_STACK_SNAPSHOT`
- `COR_PRF_MONITOR_GC`
- `COR_PRF_MONITOR_SUSPENDS`
- `COR_PRF_MONITOR_CLASS_LOADS`
- `COR_PRF_MONITOR_EXCEPTIONS`
- `COR_PRF_MONITOR_JIT_COMPILATION`
- `COR_PRF_ENABLE_REJIT`

## Useful Links

- [Pavel Yosifovich — Writing a .NET Core cross platform profiler in an hour](https://www.youtube.com/watch?v=TqS4OEWn6hQ)
- [Pavel Yosifovich — DotNext Moscou 2019 Source Code](https://github.com/zodiacon/DotNextMoscow2019)
- [Josef Biehler - Create a .NET profiler with the Profiling API](https://dev.to/gabbersepp/create-a-net-profiler-with-the-profiling-api-start-of-an-unexpected-journey-198n)
- [Mattias Högström - Building a Mixed-Mode Sampling Profiler](https://www.codeproject.com/Articles/384514/Building-a-Mixed-Mode-Sampling-Profiler)
- [Christophe Nasarre - Start a journey into the .NET Profiling APIs](https://chnasarre.medium.com/start-a-journey-into-the-net-profiling-apis-40c76e2e36cc)
- [Some random COM C++ source code](https://github.com/tenable/poc/blob/master/Comodo/Comodo%20Antivirus/ComodoInjectionCode/ComodoInjectionCode/InjectedCode.cpp)
- [Some random COM C++ source code](https://cpp.hotexamples.com/examples/-/ICLRRuntimeInfo/GetInterface/cpp-iclrruntimeinfo-getinterface-method-examples.html)
