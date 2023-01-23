using DrDotnet.Utils;
using Microsoft.Extensions.Logging.Abstractions;

public static class Program
{
    public static void Main() {
        Segfault.LoadUnload(NullLogger.Instance, PathUtils.DrDotnetBaseDirectory);
    }
}