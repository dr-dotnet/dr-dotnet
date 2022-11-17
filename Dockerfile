  FROM mcr.microsoft.com/dotnet/aspnet:6.0
  COPY pub app/
  WORKDIR /app
  ENTRYPOINT ["dotnet", "DrDotnet.Web.dll"]