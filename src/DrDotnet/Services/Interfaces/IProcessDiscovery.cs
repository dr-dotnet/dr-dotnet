using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace DrDotnet
{
    public interface IProcessDiscovery
    {
        List<ProcessInfo> GetDotnetProcesses(Action<float> progressCallback);
    }
}