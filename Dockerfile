  FROM mcr.microsoft.com/dotnet/aspnet:6.0
  COPY pub App/
  WORKDIR /App
  ENTRYPOINT ["dotnet", "DrDotnet.Web.dll"]