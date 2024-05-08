using System.Collections.Generic;
using DrDotnet.Utils;

namespace DrDotnet;

public class ProfilersDiscovery : IProfilerDiscovery
{
    public List<ProfilerInfo> GetProfilers()
    {
        var profilers = new List<ProfilerInfo>();

        var interopProfilers = NativeProfilersInterface.GetAvailableProfilers();

        profilers.AddRange(interopProfilers.Profilers);

        return profilers;
    }
}