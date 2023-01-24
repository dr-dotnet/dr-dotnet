using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace DrDotnet
{
    public interface ISessionDiscovery
    {
        List<Session> GetSessions();

        Session GetSession(Guid sessionId);

        Task<Session> AwaitUntilCompletion(Guid sessionId);
    }
}