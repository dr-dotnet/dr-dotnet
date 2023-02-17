using System.Collections.Generic;

namespace DrDotnet
{
    public interface IProfilerDiscovery
    {
        List<ProfilerInfo> GetProfilers(bool listUnreleasedProfilers = false);
    }
}