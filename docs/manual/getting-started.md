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

# Rest API
DrDotnet Web also exposes a REST API. A quite common use case is to trigger a profiling session when an issue is detected automatically. For instance, if a metric reports a likely memory leak, a memory leak analysis can be triggered automatically immediately and analyzed later by developers from the UI.

## Endpoints
Here are the REST endpoints for consuming DrDotnet Web REST API:
- POST `/api/sessions` Start a new session
- GET `/api/sessions` Return previous sessions
- GET `/api/sessions/{guid}` Return session for given guid
- GET `/api/sessions/{guid}/download` Return session reports as .zip for given guid
- GET `/api/processes` Return list of processes available to profile
- GET `/api/profilers` Return list of profilers available

In development, you can launch the Web app and navigate to http://localhost:92/swagger to find the Swagger UI with accurate and complete documentation of the API.

## Configuration
The REST API is enabled by default, but it can be disabled through an environment variable or a CLI argument. The UI can also be disabled, for instance if you want to use DrDotnet exclusively for its REST API.
- `WEB_UI_ENABLED=false` or `--no-web-ui` arg will disable web UI (only REST API)
- `REST_API_ENABLED=false` or `--no-rest-api` arg will disable REST API (only web UI)
