# 0.2.6
- Add display of profiler logs
- Add application logs (UI) to a file
- Fix logs loaded in memory that could cause high memory usage
- Fix ordering of sessions
- Fix possible segfault when detaching

# 0.2.5
- Attach to process in non-UI thread
- Change CPU sampled profiler output to html format

# 0.2.4
- Add support for raw HTML reports
- Add collapsible HTML view for *Merged Callstacks* profiler
- Fix order of stacks in *Merged Callstacks* profiler

# 0.2.3
- Add *Merged Callstacks* profiler
- Improve DuplicatedStrings output UI

# 0.2.2
- Improve *Runtime Pause* profiler to show GC information
- Improve usage of COR_PRF_HIGH_MONITOR flags

# 0.2.1
- Change Nullable Reference Types feature to true
- Improve code style

# 0.2.0
- Add support for profilers parametrization
- Change from FFIDJI to rust-protobuf for interoperability
- Change to use protobuf objects as much as possible for simplification purpose

# 0.1.1
- Remove pid namespace requirement

# 0.1.0
- Add versioning
- Add changelog 😏