using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using DrDotnet.Utils;
using FluentAssertions;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace DrDotnet.Tests.Profilers;

public class DuplicatedStringsProfilerTests : ProfilerTests
{
    protected override Guid ProfilerGuid => new Guid("{bdaba522-104c-4343-8952-036bed81527d}");

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
    public async Task Profiler_Lists_Duplicated_Strings()
    {
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();

        List<string> list = new();
        for (int i = 0; i < 666; i++)
        {
            list.Add(new string('7', 6));
        }

        Assert.True(processDiscovery.TryGetProcessInfoFromPid(Process.GetCurrentProcess().Id, out ProcessInfo? processInfo), "Could not find current process info");
        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processInfo, logger);

        await session.AwaitUntilCompletion();

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateReports().FirstOrDefault(x => x.Name == "summary.md");

        Assert.NotNull(summary, "No summary have been created!");

        var content = await File.ReadAllTextAsync(summary.FullName);

#if DEBUG
        Console.WriteLine(content);
#endif

        // We look for 77777 instead of 777777 otherwise we're going to allocate that string once more for
        // that assertion and count might not be 666 but 667
        content.Should().Contain("77777", "There should be 666 strings with 777777 content");
        content.Should().Contain("666", "There should be 666 strings with 777777 content");
    }
}
