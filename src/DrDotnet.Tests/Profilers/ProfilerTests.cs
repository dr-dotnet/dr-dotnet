using System;
using System.Linq;

namespace DrDotnet.Tests.Profilers;

public abstract class ProfilerTests
{
    public abstract Guid ProfilerGuid { get; }

    public ProfilerMetadata GetProfiler()
    {
        Logger logger = new Logger();
        ProfilersDiscovery profilersDiscovery = new ProfilersDiscovery(logger);
        var profilers = profilersDiscovery.GetProfilers(true);
        var profiler = profilers.FirstOrDefault(x => x.Guid == ProfilerGuid);

        ArgumentNullException.ThrowIfNull(profiler, $"No profiler was found with guid {ProfilerGuid}.\r\nFound profilers:\r\n {string.Join("\r\n", profilers.Select(x => $"- {x.Name} [{x.Uuid}]"))}");

        return profiler;
    }
}