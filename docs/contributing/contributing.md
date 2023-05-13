# Contributing

## How to contribute

- Checkout the [Code of Conduct](code-of-conduct.md)
- Make a Github issue
  - If you observed a bug, please indicate the reproduction steps and the Dr-Dotnet version you were using.
  - If you want to propose changes or new features
    - Checkout the [Project Spirit](##project-spirit)
    - If you want to propose a Pull Request
      - Checkout the [architecture](achitecture.md) and the [building guidelines](BUILDING.md) to get started
      - Make sure it builds and that tests are passing
      - Add changes in [Changelog](../../CHANGELOG.md) and increment version (semver). The first line is parsed and becomes the release version in the CD pipeline.
        - Prefer starting changelog entries with one of the following (in this order): Add, Update, Change, Improve, Remove or Fix.

## Project Spirit

Dr-Dotnet's goal is to fill the gap between doing *no profiling at all* and *spending countless hours on analyzing huge dumps and traces*. Thus, it is not a replacement for existing tools (WinDbg, Perfview, ...), and never it will be.     
Here are some of the most important values behind this project. The relevance of a new profiler or a feature request will be evaluated accordingly.

### Problem Focused

Usually, when it comes to profiling (with traces or memory dumps for instance), we have an objective in mind: we might have observed an unusual amount of memory usage and want to track down a memory leak, or find hot paths to optimize CPU usage, or debug a deadlock observed in production.   
**Every profiler in Dr-Dotnet is aimed at helping solve a specific issue** that could have been observed without profiling (from common observability means like a CPU metric for instance).    
Some examples are: Finding deadlocks, Detecting memory leaks, Listing CPU hotpaths, ...

### Concise

The tool should be relatively easy to use, meaning mainly two things:
- **The less parametrization we have, the better it is**. Parametrization introduces complexity and confusion to the user. If some parameter must be introduced, it must be justified, documented and set with an adequate default value for the most common scenario.
- **A profiler output must be as concise as possible**. This is not easy, because we are often tempted to output too much information, thinking that "it could be useful".    
For instance, if a profiler displays the total time spent in GC but also the longest GC pause, it could be refactored into two distinct profilers (even if the implementation won't differ much), because a high time spent in GC and a long GC pause time are two different observations.

### Performance

**Having as little overhead as possible is very important**. If the program is altered too much, the analysis may become biased. Choose wisely your profiling API keywords (some are very expensive computationally speaking) and, if possible, to the processing outside of the callbacks, ideally even once the profiler is detached.
