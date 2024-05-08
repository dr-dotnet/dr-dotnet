using System;
using System.Collections.Generic;
using System.Diagnostics;
using DrDotnet.Utils;

namespace DrDotnet;

public class ProfilersDiscovery : IProfilerDiscovery
{
    public List<ProfilerInfo> GetProfilers()
    {
        bool hideUnreleasedProfilers = Environment.GetEnvironmentVariable("HIDE_UNRELEASED") == "1";

        var profilers = new List<ProfilerInfo>();

        var interopProfilers = NativeProfilersInterface.GetAvailableProfilers();

        foreach (var interopProfiler in interopProfilers.Profilers)
        {
            if (hideUnreleasedProfilers && !interopProfiler.IsReleased)
            {
                continue;
            }
            profilers.Add(interopProfiler);
        }

        return profilers;
    }
}