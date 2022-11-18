using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;

namespace DrDotnet;

public class ProcessInfo
{
    public int Pid { get; init; }
    public string Name { get; init; }
    public string ManagedAssemblyName { get; init; }
    public string Version { get; init; }
    public DateTime StartTime { get; init; }
}