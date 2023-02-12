FROM mcr.microsoft.com/dotnet/aspnet:7.0
COPY pub app/
WORKDIR /app
# Disable diagnostics for dr-dotnet, to disambiguate it from profileable processes
ENV DOTNET_EnableDiagnostics=0
ENTRYPOINT ["dotnet", "DrDotnet.Web.dll"]