using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace DrDotnet
{
    public class ProfilersDiscovery : IProfilersDiscovery
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

            // Todo: Call dll through interop to ask for available profilers

            var interopProfilers = Interop.GetAvailableProfilers();

            foreach (var interopProfiler in interopProfilers.profilers)
            {
                profilers.Add(new Profiler {
                    Name = interopProfiler.name,
                    Guid = new Guid(interopProfiler.guid),
                    Description = interopProfiler.description
                });
            }

            return _profilers = profilers;
        }
    }
}