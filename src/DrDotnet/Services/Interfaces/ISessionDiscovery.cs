using System;
using System.Collections.Generic;

namespace DrDotnet
{
    public interface ISessionDiscovery
    {
        List<SessionInfo> GetSessions();

        SessionInfo GetSession(Guid sessionId);
    }
}