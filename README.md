# Dr-Dotnet 🩺

![build](https://github.com/ogxd/dr-dotnet/actions/workflows/build.yml/badge.svg)
![docker](https://github.com/ogxd/dr-dotnet/actions/workflows/docker.yml/badge.svg)

| WARNING: This project is still very WIP and may not *yet* fulfil general profiling needs! |
| --- |

## What is it

Dr-Dotnet is a profiling tool that can be used locally or remotely to track common issues in .NET applications such as deadlocks, cpu hotpaths, zombie threads, async hotpaths (stuck tasks), memory leaks...

## Features

- **Problem focused**<br/>The spirit of the profilers shipped with Dr-Dotnet is to target a specific issue. In other words, it won't take a full dump or a deeply nested trace and let you browse it in an attempt to find a problem among the gigabytes of data generated. Despite being the way to go in theory, in real world scenarios where applications can do a lot more than hello world or give the weather, doing so is like searching for a needle in a haystack.     Instead, the approach is to propose a few profilers whose each individual function is to look for a specific problem, such as a memory leak, a deadlock, an anormal number of exceptions, a CPU hotpath, a zombie thread, ... The output of each of these analyses can in general be summarize in a couple of line or in a table, which is perfect for an human.
- **Cross platform**<br/>Dr-Dotnet can be used to profile dotnet programs running Windows, Linux or MacOS, on X86 or ARM cpus.
- **Evolutive**<br/>Dr-Dotnet isn't really "a profiler" but rather a framework for profiling. It is shipped with a suite of builtin profilers that will grow and improve hopefully thanks to the community contributions.

## How to use

There are currently 2 recommended ways to use Dr-Dotnet, depending on your usecase:

### Dr-Dotnet Desktop

This is what you want to go for if you want to profile a dotnet program locally.    
There is no release yet but you can build it from the source.

### Dr-Dotnet as a Docker Sidecar

There is currently a CI step to build a docker image available at `ghcr.io/ogxd/drdotnet:latest`.    
This image can run on a host as a docker container, next to the container you want to profile.    
The container you want to profile must be running a dotnet program (of course) and mount a shared volume to `/tmp` to allow Dr-Dotnet to connect to the [default diagnostic port](https://learn.microsoft.com/en-us/dotnet/core/diagnostics/diagnostic-port#default-diagnostic-port).    

Create a volume (we choose to name it `diagnostics` here):
<pre>docker volume create diagnostics</pre>
Run your app:
<pre>docker run -v diagnostics:/tmp **YOUR APP IMAGE**</pre>
Then, you are ready to start Dr-Dotnet:
<pre>docker run -d --name drdotnet -v diagnostics:/tmp -p 8000:92 ghcr.io/ogxd/drdotnet:latest</pre>
You can run Dr-Dotnet anytime you want, or leave it running all the time, it won't do anything if you don't use it (just take a few mbs of RAM because of the dotnet runtime).    
Make sure the port is private to your network however for security reasons, you don't want your profiler to be open to the public.

## [How to Contribute](CONTRIBUTING.md)
## [Code of Conduct](CODE_OF_CONDUCT.md)
## [Architecture](ARCHITECTURE.md)
## [Building](BUILDING.md)

