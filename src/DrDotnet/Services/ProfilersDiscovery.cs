using System.Collections.Generic;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;

namespace DrDotnet;

public class ProfilersDiscovery : IProfilerDiscovery
{
    private ILogger _logger;
    private List<ProfilerInfo> _profilers;

    public ProfilersDiscovery(ILogger logger)
    {
        _logger = logger;
    }

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