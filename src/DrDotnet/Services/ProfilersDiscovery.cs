using DrDotnet.Interop;
using System;
using System.Collections.Generic;

namespace DrDotnet;

public class ProfilersDiscovery : IProfilerDiscovery
{
    private ILogger _logger;
    private List<Profiler> _profilers;

    public ProfilersDiscovery(ILogger logger)
    {
        _logger = logger;
    }

    public List<Profiler> GetProfilers()
    {
        if (_profilers != null)
            return _profilers;

        var profilers = new List<Profiler>();

        var interopProfilers = NativeProfilersInterface.GetAvailableProfilers();

        foreach (var interopProfiler in interopProfilers.profilers)
        {
            profilers.Add(new Profiler {
                Name = interopProfiler.name,
                ProfilerId = new Guid(interopProfiler.guid),
                Description = interopProfiler.description
            });
        }

        return _profilers = profilers;
    }
}