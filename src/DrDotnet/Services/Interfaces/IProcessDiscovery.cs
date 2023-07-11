using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;

namespace DrDotnet
{
    public interface IProcessDiscovery
    {
        List<ProcessInfo> GetDotnetProcesses(Action<float>? progressCallback = null);
        bool TryGetProcessInfoFromPid(int pid, [NotNullWhen(true)] out ProcessInfo? processInfo);
    }
}