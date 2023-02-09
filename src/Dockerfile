FROM mcr.microsoft.com/dotnet/aspnet:7.0
COPY pub app/
WORKDIR /app
# This is a trick to have a different PID for the DrDotnet process (instead of 1)
#CMD ["/bin/bash", "dotnet", "DrDotnet.Web.dll"]
ENTRYPOINT ["dotnet", "DrDotnet.Web.dll"]