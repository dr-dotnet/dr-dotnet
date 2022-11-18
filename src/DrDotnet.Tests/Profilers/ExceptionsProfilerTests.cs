using NUnit.Framework;
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace DrDotnet.Tests.Profilers;

public class ExceptionsProfilerTests : ProfilerTests
{
    public override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-F785C3186E82}");

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
    [Timeout(30_000)]
    [NonParallelizable]
    public async Task Profiler_Counts_Exceptions()
    {
        Logger logger = new Logger();
        SessionsDiscovery sessionsDiscovery = new SessionsDiscovery(logger);
        Profiler profiler = GetProfiler();

        Guid sessionId = profiler.StartProfilingSession(Process.GetCurrentProcess().Id, logger);

        // Intentionally throws (handled) exceptions
        ThreadPool.QueueUserWorkItem(async _ =>
        {
            while (true)
            {
                try
                {
                    throw new TestException();
                }
                catch { }
                await Task.Delay(300);
            }
        });

        var session = await sessionsDiscovery.AwaitUntilCompletion(sessionId);

        var summary = session.EnumerateFiles().Where(x => x.Name == "summary.md").FirstOrDefault();

        Assert.NotNull(summary, "No summary have been created!");

        var content = File.ReadAllText(summary.FullName);

        Console.WriteLine(content);

        Assert.IsTrue(content.Contains("DrDotnet.Tests.Profilers.TestException:"));
        Assert.IsFalse(content.Contains("DrDotnet.Tests.Profilers.TestException: 0"));
    }
}

public class TestException : Exception { }