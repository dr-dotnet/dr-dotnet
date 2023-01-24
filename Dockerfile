FROM mcr.microsoft.com/dotnet/aspnet:7.0
COPY pub app/
WORKDIR /app
ENTRYPOINT ["dotnet", "DrDotnet.Web.dll"]