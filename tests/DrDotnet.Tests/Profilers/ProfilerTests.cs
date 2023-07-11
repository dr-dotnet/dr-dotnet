using System;
using System.Linq;

namespace DrDotnet.Tests.Profilers;

public abstract class ProfilerTests
{
    protected abstract Guid ProfilerGuid { get; }

    protected ProfilerInfo GetProfiler()
    {
        ProfilersDiscovery profilersDiscovery = new ProfilersDiscovery();
        var profilers = profilersDiscovery.GetProfilers(true);
        var profiler = profilers.FirstOrDefault(x => x.Guid == ProfilerGuid);

        ArgumentNullException.ThrowIfNull(profiler, $"No profiler was found with guid {ProfilerGuid}.\r\nFound profilers:\r\n {string.Join("\r\n", profilers.Select(x => $"- {x.Name} [{x.Uuid}]"))}");

        return profiler;
    }
}