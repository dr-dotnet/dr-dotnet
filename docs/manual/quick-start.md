# Quick Start



There is currently a CI step to build a docker image available on [docker hub](https://hub.docker.com/r/drdotnet) at `drdotnet/web:latest`.    
This image can run on a host as a docker container, next to the container you want to profile.    
The container you want to profile must be running a dotnet program (of course) and mount a shared volume to `/tmp` to allow Dr-Dotnet to connect to the [default diagnostic port](https://learn.microsoft.com/en-us/dotnet/core/diagnostics/diagnostic-port#default-diagnostic-port).    

Create a volume (we choose to name it `diagnostics` here):
<pre>docker volume create diagnostics</pre>
Run your app:
<pre>docker run -v diagnostics:/tmp **YOUR APP IMAGE**</pre>
Then, you are ready to start Dr-Dotnet:
<pre>docker run -d --name drdotnet -v diagnostics:/tmp -p 8000:92 drdotnet/web:latest</pre>
You can run Dr-Dotnet anytime you want, or leave it running all the time, it won't do anything if you don't use it (just take a few mbs of RAM because of the dotnet runtime).    
Make sure the port is private to your network however for security reasons, you don't want your profiler to be open to the public.