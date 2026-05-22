using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Threading.Tasks;
using DrDotnet.Tests.Simulations;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace DrDotnet.Tests.Profilers;

public class AsyncHotpathProfilerTests : ProfilerTests
{
    protected override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-A283B2056E79}");

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
    public async Task Profiler_Lists_Async_Hotpaths()
    {
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();
        profiler.SetParameter("duration_seconds", 10);
        
        bool profiling = true;
        
        var tasks = Enumerable.Repeat(0, 10_000).Select(async _ => 
        {
            while (profiling)
            {
                await DoWork1Async();
                await DoWork2Async();
                await DoWork3Async();
            }
        }).ToArray();
  
        Assert.True(processDiscovery.TryGetProcessInfoFromPid(Process.GetCurrentProcess().Id, out ProcessInfo? processInfo), "Could not find current process info");
        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processInfo, logger);

        await session.AwaitUntilCompletion();

        profiling = false;
        
        await Task.WhenAll(tasks);

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateReports().FirstOrDefault(x => x.Name == "async_hotpaths.html");

        Assert.NotNull(summary, "No summary have been created!");

        var content = await File.ReadAllTextAsync(summary.FullName);
        
        Console.WriteLine(content);
        
        // Todo: Add assertions
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private async Task DoWork1Async()
    {
        for (int i = 0; i < 10; i++)
        {
            await Task.Delay(20);
        }
    }
    
    [MethodImpl(MethodImplOptions.NoInlining)]
    private async Task DoWork2Async()
    {
        await Task.Delay(20);
    }
    
    [MethodImpl(MethodImplOptions.NoInlining)]
    private async Task DoWork3Async()
    {
        await DoWork2Async();
        await Task.Yield();
    }
}
