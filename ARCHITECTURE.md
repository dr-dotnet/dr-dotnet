# Architecture

## Overview

The .NET Profiling API is accessible via COM interop (cross-platform thanks to the Platform Adaptation Layer) and allows little overheap profiling compared to other methods while giving a wide range of possibilities for profiling purpose. Perfview uses this API (on top of ETW) however it does it from managed code calling mocked COM objects written in C#.     

In this Dr-Dotnet, we're using **Rust** for coding the profilers for the safety and the modernity of the language. The CLR profiling API rust binding are originally taken from [this project from Camden Reslink](https://github.com/camdenreslink/clr-profiler) who did a really great job.    
The UI and profilers management are coded in C#, for the productivity the language offers and because it is convenient for interperability with Rust. Bindings between C# and Rust are autogenerated using [FFIDJI](https://github.com/ogxd/ffidji) (but they are fairly trivial for now, to be honest this is probably overkill).

Here is a diagram of the profiling workflow:
```mermaid
sequenceDiagram

participant user as End User
participant drdotnet as Dr-Dotnet UI (Web or Desktop)
participant app as Your Dotnet Application(s)
participant profilers as Profilers Library

user->>+drdotnet: List dotnet processes
drdotnet->>+app: Request process(es) PID(s)
app-->>-drdotnet: PID(s)
drdotnet-->>-user: Display list of processes

user->>+drdotnet: List profilers
drdotnet->>+profilers: Request available profilers through interop (C# / Rust)
profilers-->>-drdotnet: List of profilers
drdotnet-->>-user: Display list of profilers

user->>+drdotnet: Run given profiler on given process
drdotnet->>profilers: Copy library in folder that is accessible by targeted process
drdotnet->>+app: Request CLR to attach profilers from library
app->>-profilers: Load, instantiate and initialize profiler
activate profilers
opt Actual profiler implementation
app->>profilers: Some callback
app->>profilers: Some callback
app->>profilers: Some callback
end
profilers-->>-app: Request detach
app->>+profilers: Detached callback
profilers->>profilers: Create session files
profilers-->>-drdotnet: Session files are available
drdotnet-->>-user: Display profiling session results
```

The following sections will cover some part of that workflow more in depth.

## C# / Rust Interoperability

To be documented.

## Loading, instantiating and initializing the profilers library

To be documented.

## Docker subtleties

To be documented.

## The Rust bindings on the CLR profiling API

To be documented.