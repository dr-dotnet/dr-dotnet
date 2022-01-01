using System.Collections.Generic;

namespace DrDotnet
{
    public interface IProfilersDiscovery
    {
        List<Profiler> GetProfilers();
    }
}