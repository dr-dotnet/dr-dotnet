using System.Collections.Generic;

namespace DrDotnet
{
    public interface IProfilerDiscovery
    {
        List<ProfilerMetadata> GetProfilers(bool listUnreleasedProfilers = false);
    }
}