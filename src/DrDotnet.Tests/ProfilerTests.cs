using NUnit.Framework;
using System;
using System.Linq;

namespace DrDotnet.Tests;

public abstract class ProfilerTests
{
    public abstract Guid ProfilerGuid { get; }

    public Profiler GetProfiler()
    {
        ILogger logger = new Logger();
        ProfilersDiscovery profilersDiscovery = new ProfilersDiscovery(logger);
        var profilers = profilersDiscovery.GetProfilers(true);
        var profiler = profilers.Where(x => x.ProfilerId == ProfilerGuid).FirstOrDefault();

        ArgumentNullException.ThrowIfNull(profiler, $"No profiler was found with guid {ProfilerGuid}.\r\nFound profilers:\r\n {string.Join("\r\n", profilers.Select(x => $"- {x.Name} [{x.ProfilerId}]"))}");

        return profiler;
    }
}