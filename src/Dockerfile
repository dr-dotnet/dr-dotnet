FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
# Rust toolchain for the native profiler build
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
RUN dotnet publish src/DrDotnet.Web/DrDotnet.Web.csproj -c Release -o /app \
    && cp bin/release/libprofilers.so /app/libprofilers.so

FROM mcr.microsoft.com/dotnet/aspnet:8.0
COPY --from=build /app /app/
WORKDIR /app
# Disable diagnostics so dr-dotnet itself isn't profileable
ENV DOTNET_EnableDiagnostics=0
ENTRYPOINT ["dotnet", "DrDotnet.Web.dll"]
