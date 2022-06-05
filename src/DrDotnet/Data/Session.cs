using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;

namespace DrDotnet;

public class Session
{
    public Guid SessionId { get; set; }

    public string ProcessName { get; set; }

    public DateTime Timestamp { get; set; }

    public Profiler Profiler { get; set; }

    private string _sessionFilePath;

    public const string SESSION_FILE_NAME = "session.json";

    public IEnumerable<FileInfo> EnumerateFiles()
    {
        return new FileInfo(_sessionFilePath).Directory.EnumerateFiles();
    }

    public static Session FromPath(string sessionFilePath)
    {
        var options = new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true
        };

        if (!File.Exists(sessionFilePath))
            throw new FileNotFoundException($"There is no session file at path '{sessionFilePath}'");

        var jsonString = File.ReadAllText(sessionFilePath);
        var session = JsonSerializer.Deserialize<Session>(jsonString, options);
        session._sessionFilePath = sessionFilePath;

        return session;
    }
}