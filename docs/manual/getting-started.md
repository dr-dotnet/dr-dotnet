# Getting Started

There are two ways to use Dr-Dotnet:
- As a **desktop application**, for profiling local dotnet applications
- As a **web application**, shipped as a docker container, for profiling remote dotnet applications

## Using the desktop version

Check out the [Releases](https://github.com/dr-dotnet/dr-dotnet/releases) to download the latest version of Dr-Dotnet for your platform.

The desktop application is a standalone executable bundled in a .zip file. Extract the archive and run the executable.

> **Note**
> The binaries are currently unsigned. That can be an issue for MacOS which, by default, prevents unsigned binaries from running.

## Using the web app version

Official docker images are available on [Docker Hub](https://hub.docker.com/r/drdotnet/web/tags).

To be able to profile a remote dotnet application, you need to run Dr-Dotnet as a docker side-car, meaning that it would run on the same host as the application you want to profile, but as a separate container. 

For Dr-Dotnet to be able to find other dotnet containers running on that same host, both containers must map a shared volume as their `/tmp` folder.

Here is an example of how to set up Dr-Dotnet to profile a dotnet application running in a docker container:

- Create a volume (we choose to name it `diagnostics` here):
```bash
docker volume create diagnostics
```
- Run your app, with the volume mounted as `/tmp`:
```bash
docker run -v diagnostics:/tmp <YOUR APP IMAGE>
```
- Then, you are ready to start Dr-Dotnet:
```bash
docker run -d --name drdotnet -v diagnostics:/tmp -p 8000:92 drdotnet/web:latest
```
Then go ahead and open your browser at `http://localhost:8000`

> **Note**
> When not used, the only overhead from leaving a Dr-Dotnet container alive are the few MBs of memory it uses (mostly because of the dotnet runtime). It is safe to leave it running on your host.   

> **Warning**
> Make sure the port is private to your network for security reasons, you don't want your profiler to be open to the public.

## Using web app version - Kubernetes

This is possible, but currently, the workflow is a little rough. We are working on making it easier. Please come back later :)