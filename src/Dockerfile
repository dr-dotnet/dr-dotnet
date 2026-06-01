# ---- Build stage: builds the managed app AND the native Rust profiler in-image ----
# Built once per target platform by `docker buildx`; the native profiler therefore
# always matches the image architecture (the CLR loads it in-process).
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build

# Rust toolchain: the DrDotnet.csproj pre-build step compiles libprofilers.so with cargo.
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates build-essential \
    && rm -rf /var/lib/apt/lists/* \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /src
COPY . .

ARG VERSION=0.0.0
ARG BUILD_NUMBER=0
ENV VERSION=${VERSION} BUILD_NUMBER=${BUILD_NUMBER}

# `dotnet publish` builds the portable managed app and triggers the cargo build of
# the native profiler (csproj pre-build) into bin/release/libprofilers.so.
RUN dotnet publish src/DrDotnet.Web/DrDotnet.Web.csproj -c Release -o /app \
    && cp bin/release/libprofilers.so /app/libprofilers.so

# ---- Runtime stage ----
FROM mcr.microsoft.com/dotnet/aspnet:8.0
COPY --from=build /app /app/
WORKDIR /app
# Disable diagnostics for dr-dotnet, to disambiguate it from profileable processes
ENV DOTNET_EnableDiagnostics=0
ENTRYPOINT ["dotnet", "DrDotnet.Web.dll"]
