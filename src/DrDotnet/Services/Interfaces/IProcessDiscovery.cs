using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;

namespace DrDotnet
{
    public interface IProcessDiscovery
    {
        ValueTask<List<ProcessInfo>> GetDotnetProcessesAsync(Action<float> progressCallback);
    }
}