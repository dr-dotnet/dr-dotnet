FROM mcr.microsoft.com/dotnet/aspnet:8.0
# TARGETARCH is provided automatically by `docker buildx` (e.g. "amd64", "arm64").
# The native profiler library is architecture-specific: the CLR loads it in-process
# into the profiled application, so its CPU architecture MUST match the image's.
# Each architecture's library is built outside the image and selected here.
ARG TARGETARCH
COPY pub app/
COPY libprofilers-${TARGETARCH}.so app/libprofilers.so
WORKDIR /app
# Disable diagnostics for dr-dotnet, to disambiguate it from profileable processes
ENV DOTNET_EnableDiagnostics=0
ENTRYPOINT ["dotnet", "DrDotnet.Web.dll"]
