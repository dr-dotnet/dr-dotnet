# Guidelines

Dr. Dotnet's approach to profiling is an analogy to visiting the doctor: you usually expose your symptoms (headache, cough, ...) to a doctor, and he makes his short and human-readable diagnosis.

In the context of a dotnet application, the symptoms are unwanted things you can observe from simple metrics (high CPU usage, memory leak, ...). From that observation, by using the right profiler(s), Dr. Dotnet will output a short and problem-focused diagnosis.

## What profiler to use?

Here are some non-exhaustive recommendations on what to do depending on observations:
| Observations | Profiler to use | Additional recommendations |
|---|---|---|
| Gen 2 memory increases over time, and hardly only goes down with a restart. This is a sign of a **memory leak**. | GC Survivors | GC Survivors profiler will list objects in Gen 2 along with their most prominent retention paths. With this, any substantial memory leak should be visible: a high number of leaked objects will show up, and a very large object leaking as well (a lot of bytes will be retained) |
| CPU usage is (abnormally) high | CPU Hotpaths | Identify CPU hotpaths with this rather classic sampled CPU usage profiling method. |
| Thread starvation</br>High number of pending work item</br>Suspected deadlock | Merged Call Stacks | A merged call stacks view will show where each thread is at, merged by common call stacks. With this you should be able to see if a large number of threads are stuck on a given operation |
| High 95p / 99p latency on a web API but low average latency | Runtime Pauses | It is likely that your application is spending a lot of time in GC. Pauses for GC may be rare, very long pauses can destroy your 95p / 99p SLOs. Runtime Pauses will show you the GC pauses and their duration. This won't help you find the root cause, but it might help you confirm this hypothesis. If you do observe long pauses, then you may want to check the "High allocation rate" case |
| High memory usage for an app that deals with a lot of strings | Duplicated Strings | It is common for a dotnet app to have duplicated strings in memory (two or more strings with the same value, thus occupying more space than a single one would require). Use this profiler to see how much memory is wasted from such strings. The profiler will display their value, hopefully, it will help you find where these strings belong to begin your optimization work. A common solution is [string interning](https://learn.microsoft.com/en-us/dotnet/api/system.string.intern?view=net-7.0). |
| High memory usage | GC Survivors | The GC survivors profiler will display major retention paths for gen 2 objects, along with retained bytes. This should help you identify which parts of your application uses the most memory without needing to dump the whole memory. |
| Time spent in GC (abnormally) high</br>High allocation rate (via ETW) | Allocations by class (WIP) |  |

## Need to go deeper?

DrDotnet will make it easy to pinpoint common issues, but sometimes, you need to go deeper. In that case, you can use the following tools/resources:
- [mem-doc by Maoni Stephens](https://github.com/Maoni0/mem-doc/blob/master/doc/.NETMemoryPerformanceAnalysis.md) for in-depth memory performance analysis
- [dotnet-trace](https://docs.microsoft.com/en-us/dotnet/core/diagnostics/dotnet-trace) for in-depth cpu time analysis
- [dotnet-dump](https://docs.microsoft.com/en-us/dotnet/core/diagnostics/dotnet-dump) for in-depth memory analysis
- [JetBrains dotMemory (commercial)](https://www.jetbrains.com/dotmemory/?source=google&medium=cpc&campaign=12509621702&term=dotmemory&content=504866862913&gad=1&gclid=CjwKCAjwp6CkBhB_EiwAlQVyxSKUJ6qk5EIm18hjQzuar_1wT-todzOGCkCQwu3Z6jZWvF_Bxg0d1hoCm0UQAvD_BwE)