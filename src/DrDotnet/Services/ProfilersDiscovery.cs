using System;
using System.Collections.Generic;
using System.Diagnostics;
using DrDotnet.Utils;

namespace DrDotnet;

public class ProfilersDiscovery : IProfilerDiscovery
{
    public List<ProfilerInfo> GetProfilers()
    {
        var profilers = new List<ProfilerInfo>();

        var interopProfilers = NativeProfilersInterface.GetAvailableProfilers();

        foreach (var interopProfiler in interopProfilers.Profilers)
        {
            profilers.Add(interopProfiler);
        }

        return profilers;
    }
}