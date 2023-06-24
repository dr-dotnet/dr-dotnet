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

        //string output = process.StandardOutput.ReadToEnd();
        string errors = process.StandardError.ReadToEnd();

        process.WaitForExit();

        // Assert that the process completed successfully
        Assert.That(process.ExitCode, Is.EqualTo(0), errors);
    }
}