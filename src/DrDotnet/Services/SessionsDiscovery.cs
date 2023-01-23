using System;
using System.Collections.Generic;
using System.IO;
using System.Threading.Tasks;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;

namespace DrDotnet;

public class SessionsDiscovery : ISessionDiscovery
{
    private ILogger _logger;

    public SessionsDiscovery(ILogger logger)
    {
        _logger = logger;
    }

    public List<Session> GetSessions()
    {
        var sessions = new List<Session>();

        string[] subdirectoryEntries = Directory.GetDirectories(PathUtils.DrDotnetBaseDirectory);
        foreach (string subdirectory in subdirectoryEntries)
        {
            string sessionFilePath = Path.Combine(subdirectory, Session.SESSION_FILE_NAME);
            try
            {
                sessions.Add(Session.FromPath(sessionFilePath));
            }
            catch (Exception e)
            {
                _logger.LogError(e, "Error while retreiving session at path '{SessionPath}'", sessionFilePath);
            }
        }

        return sessions;
    }

    public Session GetSession(Guid sessionId)
    {
        return Session.FromPath(GetSessionPath(sessionId));
    }

    private string GetSessionPath(Guid sessionId)
    {
        return Path.Combine(Path.Combine(PathUtils.DrDotnetBaseDirectory, sessionId.ToString()), Session.SESSION_FILE_NAME);
    }

    public async Task<Session> AwaitUntilCompletion(Guid sessionId)
    {
        var sessionFilePath = GetSessionPath(sessionId);
        while (!File.Exists(sessionFilePath))
        {
            // Wait until the session manifest has been written
            await Task.Delay(1000);
        }

        return GetSession(sessionId);
    }
}