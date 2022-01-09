using System.Collections.Generic;

namespace DrDotnet
{
    public interface ISessionDiscovery
    {
        List<Session> GetSessions();
    }
}