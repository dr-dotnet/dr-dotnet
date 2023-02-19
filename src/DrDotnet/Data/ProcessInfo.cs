namespace DrDotnet;

public class ProcessInfo
{
    public required int Id { get; init; }
    public required string ManagedAssemblyName { get; init; }
    public required string Version { get; init; }
}