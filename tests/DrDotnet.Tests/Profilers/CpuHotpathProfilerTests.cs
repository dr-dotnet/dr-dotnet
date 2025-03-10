using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using DrDotnet.Tests.Simulations;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace DrDotnet.Tests.Profilers;

public class CpuHotpathProfilerTests : ProfilerTests
{
    protected override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-A485B2056E71}");

    [Test]
    [Order(0)]
    [Timeout(5_000)]
    [NonParallelizable]
    public void Profiler_Exists()
    {
        Assert.NotNull(GetProfiler());
    }

    [Test, Explicit]
    [Order(1)]
    [Timeout(160_000)]
    [NonParallelizable]
    public async Task Profiler_Lists_Cpu_Hotpaths()
    {
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();
        profiler.SetParameter("duration_seconds", 10);
        
        using var service1 = new FibonacciSimulation();
        using var service2 = new FibonacciSimulation();
        using var service3 = new FibonacciSimulation();
        using var service4 = new FibonacciSimulation();
        
        await Task.Delay(3000);
  
        Assert.True(processDiscovery.TryGetProcessInfoFromPid(Process.GetCurrentProcess().Id, out ProcessInfo? processInfo), "Could not find current process info");
        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processInfo, logger);

        await session.AwaitUntilCompletion();

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateReports().FirstOrDefault(x => x.Name == "cpu_hotpaths.html");

        Assert.NotNull(summary, "No summary have been created!");

        var content = await File.ReadAllTextAsync(summary.FullName);

#if DEBUG
        Console.WriteLine(content);
#endif
        
        // Todo: Add assertions
    }
    
    [Test, Explicit]
    [Order(1)]
    [Timeout(160_000)]
    [NonParallelizable]
    public async Task Profiler_Lists_Cpu_Hotpaths_Generics()
    {
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();
        profiler.SetParameter("duration_seconds", 10);
        profiler.SetParameter("filter_suspended_threads", false);
        profiler.SetParameter("try_resolve_generics", true);
        
        using var service1 = new FibonacciGeneric<int>();
        using var service2 = new FibonacciGeneric<List<int>>();
        using var service3 = new FibonacciGeneric<bool>();
        using var service4 = new FibonacciGeneric<Dictionary<string, int>>();
        
        await Task.Delay(3000);
  
        Assert.True(processDiscovery.TryGetProcessInfoFromPid(Process.GetCurrentProcess().Id, out ProcessInfo? processInfo), "Could not find current process info");
        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processInfo, logger);

        await session.AwaitUntilCompletion();
        
        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateReports().FirstOrDefault(x => x.Name == "cpu_hotpaths.html");

        Assert.NotNull(summary, "No summary have been created!");

        var content = await File.ReadAllTextAsync(summary.FullName);
        
        Console.WriteLine(content);
        
        // Todo: Add assertions
    }
}
