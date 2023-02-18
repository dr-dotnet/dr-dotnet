using System;
using System.Collections.Generic;

namespace DrDotnet
{
    public interface IProcessDiscovery
    {
        List<ProcessInfo> GetDotnetProcesses(Action<float> progressCallback);
        ProcessInfo GetProcessInfoFromPid(int pid);
    }
}