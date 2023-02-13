using DrDotnet.Interop;
using System;
using System.Collections.Generic;
using Microsoft.Extensions.Logging;

namespace DrDotnet;

public class ProfilersDiscovery : IProfilerDiscovery
{
    private ILogger _logger;
    private List<Profiler> _profilers;

    public ProfilersDiscovery(ILogger logger)
    {
        _logger = logger;
    }

    public List<Profiler> GetProfilers(bool listUnreleasedProfilers = false)
    {
        if (_profilers != null)
            return _profilers;

        var profilers = new List<Profiler>();

        var interopProfilers = NativeProfilersInterface.GetAvailableProfilers();

        foreach (var interopProfiler in interopProfilers.Profilers)
        {
            if (listUnreleasedProfilers || interopProfiler.IsReleased)
            {
                profilers.Add(new Profiler
                {
                    Name = interopProfiler.Name,
                    ProfilerId = new Guid(interopProfiler.Uuid),
                    Description = interopProfiler.Description
                });
            }
        }

        return _profilers = profilers;
    }
}