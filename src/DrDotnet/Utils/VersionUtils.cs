using System;
using System.Reflection;

namespace DrDotnet.Utils;

public static class VersionUtils
{
    public static Version CurrentVersion => Assembly.GetEntryAssembly()!.GetName().Version;
}