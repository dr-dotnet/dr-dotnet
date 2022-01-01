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

            profilers.Add(new Profiler { Name = "Memory Leak Detector", Guid = Guid.NewGuid(), Description = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." });
            profilers.Add(new Profiler { Name = "Synchronous Hot Paths", Guid = Guid.NewGuid(), Description = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." });

            _logger.Log($"Profilers native call: {GetProfilersInternal()}");

            return _profilers = profilers;
        }

        [DllImport("profiler.dll", EntryPoint = "string_from_rust", CharSet = CharSet.Ansi, SetLastError = false)]
        private static extern String GetProfilersInternal();
    }
}