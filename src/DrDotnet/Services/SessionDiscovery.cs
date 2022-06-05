using System;
using System.Collections.Generic;
using System.IO;
using System.Threading.Tasks;

namespace DrDotnet;

public class SessionDiscovery : ISessionDiscovery
{
    private ILogger _logger;

    public SessionDiscovery(ILogger logger)
    {
        _logger = logger;
    }

    public string RootDir {
        get {
            var dir = Path.Combine(/*_env.ContentRootPath*/ "C:\\", "dr-dotnet");
            Directory.CreateDirectory(dir);
            return dir;
        }
    }

    public List<Session> GetSessions()
    {
        var sessions = new List<Session>();

        string[] subdirectoryEntries = Directory.GetDirectories(RootDir);
        foreach (string subdirectory in subdirectoryEntries)
        {
            string sessionFilePath = Path.Combine(subdirectory, Session.SESSION_FILE_NAME);
            try
            {
                sessions.Add(Session.FromPath(sessionFilePath));
            }
            catch (Exception ex)
            {
                _logger.Log(ex.ToString());
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
        return Path.Combine(Path.Combine(RootDir, sessionId.ToString()), Session.SESSION_FILE_NAME);
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