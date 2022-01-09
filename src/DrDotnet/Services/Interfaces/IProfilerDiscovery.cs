using System.Collections.Generic;

namespace DrDotnet
{
    public interface IProfilerDiscovery
    {
        List<Profiler> GetProfilers();
    }
}