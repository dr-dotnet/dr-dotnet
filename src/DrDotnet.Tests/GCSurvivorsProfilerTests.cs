using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace DrDotnet.Tests;

public class GCSurvivorsProfilerTests : ProfilerTests
{
    public override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-F785C3186E86}");

    [Test]
    [Order(0)]
    [Timeout(5_000)]
    [NonParallelizable]
    public void Profiler_Exists()
    {
        Assert.NotNull(GetProfiler());
    }

    [Test]
    [Order(1)]
    [Timeout(160_000)]
    [NonParallelizable]
    public async Task Profiler_Detects_GC_Survivors_Referenced_From_Gen2()
    {
        ILogger logger = new Logger();
        SessionDiscovery sessionDiscovery = new SessionDiscovery(logger);
        Profiler profiler = GetProfiler();

        using var service = new MyService(1_000_000, 100_000);
        await Task.Delay(3000);

        Guid sessionId = profiler.StartProfilingSession(Process.GetCurrentProcess().Id, logger);

        var session = await sessionDiscovery.AwaitUntilCompletion(sessionId);

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateFiles().Where(x => x.Name == "summary.md").FirstOrDefault();
        
        Assert.NotNull(summary, "No summary have been created!");
        
        var content = File.ReadAllText(summary.FullName);
        
        Console.WriteLine(content);
    }
}

public class MyService : IDisposable
{
    public readonly int _allocatedObjectsPerSecond = 10_0;
    public readonly int _maxAliveObjects = 2_000;

    private readonly Queue<string> _queue = new Queue<string>();

    private volatile bool _disposed = false;

    public MyService(int allocatedObjectsPerSecond, int maxAliveObjects)
    {
        _allocatedObjectsPerSecond = allocatedObjectsPerSecond;
        _maxAliveObjects = maxAliveObjects;

        ThreadPool.QueueUserWorkItem((_) =>
        {
            Span<char> strspan = stackalloc char[50];

            Stopwatch sw = Stopwatch.StartNew();

            int i = 0;

            while (!_disposed)
            {
                if (sw.Elapsed.TotalSeconds < i * 1d / _allocatedObjectsPerSecond)
                {
                    Thread.Sleep(10);
                }

                for (int j = 0; j < strspan.Length; j++)
                {
                    strspan[j] = (char)Random.Shared.Next(48, 122);
                }

                _queue.Enqueue(new string(strspan));

                if (_queue.Count > _maxAliveObjects)
                {
                    _queue.Dequeue();
                }

                i++;
            }
        });
    }

    public void Dispose()
    {
        _disposed = true;
    }
}