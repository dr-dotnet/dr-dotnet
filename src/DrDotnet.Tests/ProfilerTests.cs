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
        var profilers = profilersDiscovery.GetProfilers();
        return profilers.Where(x => x.ProfilerId == ProfilerGuid).FirstOrDefault();
    }
}