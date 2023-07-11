using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;

namespace DrDotnet
{
    public interface ISessionDiscovery
    {
        List<SessionInfo> GetSessions();

        bool TryGetSession(Guid sessionId, [NotNullWhen(true)] out SessionInfo? sessionInfo);
    }
}