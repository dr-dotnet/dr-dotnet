using System;
using System.Diagnostics;
using NUnit.Framework;

namespace DrDotnet.Profilers.Tests;

public class ProfilersTests
{
    [Test]
    public void ProfilersTests_ArePassing()
    {
        var process = new Process()
        {
            StartInfo = new ProcessStartInfo
            {
                FileName = "cargo",
                Arguments = "test",
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                UseShellExecute = false,
                CreateNoWindow = true,
                WorkingDirectory = "../../src/DrDotnet.Profilers",
            }
        };

        process.Start();
        process.WaitForExit();
        
        Console.WriteLine(process.StandardOutput.ReadToEnd());

        // Assert that the process completed successfully
        Assert.That(process.ExitCode, Is.EqualTo(0), process.StandardError.ReadToEnd());
    }
}