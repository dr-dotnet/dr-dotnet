using System.Collections.Generic;
using DrDotnet.Utils;

namespace DrDotnet;

public class ProfilersDiscovery : IProfilerDiscovery
{
    private List<ProfilerInfo>? _profilers;

    public List<ProfilerInfo> GetProfilers(bool listUnreleasedProfilers = false)
    {
        if (_profilers != null)
            return _profilers;

        var profilers = new List<ProfilerInfo>();

        var interopProfilers = NativeProfilersInterface.GetAvailableProfilers();

        foreach (var interopProfiler in interopProfilers.Profilers)
        {
            if (listUnreleasedProfilers || interopProfiler.IsReleased)
            {
                profilers.Add(interopProfiler);
            }
        }

        return _profilers = profilers;
    }
}