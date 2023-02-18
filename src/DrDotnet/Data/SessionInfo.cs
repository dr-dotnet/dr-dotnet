using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Text.Json;
using System.Threading.Tasks;
using DrDotnet.Utils;

public partial class SessionInfo
{
    public const string SESSION_FILE_NAME = "session.json";

    public SessionInfo(ProfilerInfo profiler, string processName)
    {
        Uuid = Guid.NewGuid().ToString();
        Profiler = profiler;
        ProcessName = processName;
        Timestamp = DateTime.UtcNow.ToString(CultureInfo.InvariantCulture);
    }
    
    public Guid SessionId => new(Uuid);

    public DateTime TimestampDate => DateTime.Parse(Timestamp, CultureInfo.InvariantCulture);

    public IEnumerable<FileInfo> EnumerateFiles()
    {
        return new FileInfo(Path).Directory.EnumerateFiles();
    }

    public string Path => GetPath(SessionId);

    public static SessionInfo FromPath(string sessionFilePath)
    {
        var options = new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true
        };

        if (!File.Exists(sessionFilePath))
            throw new FileNotFoundException($"There is no session file at path '{sessionFilePath}'");

        var jsonString = File.ReadAllText(sessionFilePath);
        var session = JsonSerializer.Deserialize<SessionInfo>(jsonString, options);

        return session;
    }
    
    public static string GetPath(Guid sessionId)
    {
        return System.IO.Path.Combine(PathUtils.DrDotnetBaseDirectory, sessionId.ToString(), SESSION_FILE_NAME);
    }

    public bool IsCompleted => File.Exists(Path);
    
    public async Task AwaitUntilCompletion()
    {
        while (!IsCompleted)
        {
            // Wait until the session manifest has been written
            await Task.Delay(500);
        }
    }
}