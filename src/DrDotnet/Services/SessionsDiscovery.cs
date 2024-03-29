﻿using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.IO;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;

namespace DrDotnet;

public class SessionsDiscovery : ISessionDiscovery
{
    private readonly ILogger _logger;

    public SessionsDiscovery(ILogger<SessionsDiscovery> logger)
    {
        _logger = logger;
    }

    public List<SessionInfo> GetSessions()
    {
        var sessions = new List<SessionInfo>();

        string[] subdirectoryEntries = Directory.GetDirectories(PathUtils.DrDotnetBaseDirectory);
        foreach (string subdirectory in subdirectoryEntries)
        {
            string sessionFilePath = Path.Combine(subdirectory, SessionInfo.SESSION_FILE_NAME);
            try
            {
                sessions.Add(SessionInfo.FromPath(sessionFilePath));
            }
            catch (Exception e)
            {
                _logger.LogError(e, "Error while retreiving session at path '{SessionPath}'", sessionFilePath);
            }
        }

        return sessions;
    }

    public bool TryGetSession(Guid sessionId, [NotNullWhen(true)] out SessionInfo? sessionInfo)
    {
        try
        {
            sessionInfo = SessionInfo.FromPath(SessionInfo.GetPath(sessionId))!;
            return true;
        }
        catch
        {
            sessionInfo = null;
            return false;
        }
    }
}