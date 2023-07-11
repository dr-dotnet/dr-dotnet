using System.Collections.Generic;
using DrDotnet.Utils;

namespace DrDotnet;

public class ProfilersDiscovery : IProfilerDiscovery
{
    public List<ProfilerInfo> GetProfilers(bool listUnreleasedProfilers = false)
    {
        var profilers = new List<ProfilerInfo>();

        var interopProfilers = NativeProfilersInterface.GetAvailableProfilers();

        foreach (var interopProfiler in interopProfilers.Profilers)
        {
            if (listUnreleasedProfilers || interopProfiler.IsReleased)
            {
                profilers.Add(interopProfiler);
            }
        }

        return profilers;
    }
}