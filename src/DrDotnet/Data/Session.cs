using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;

namespace DrDotnet
{
    public class Session
    {
        public Guid SessionId { get; set; }

        public string ProcessName { get; set; }

        public DateTime Timestamp { get; set; }

        public Profiler Profiler { get; set; }

        public const string SESSION_ROOT_DIR = "dr-dotnet";
        public const string SESSION_FILE_NAME = "session.json";

        public IEnumerable<FileInfo> EnumerateFiles()
        {
            return new DirectoryInfo($"/{SESSION_ROOT_DIR}/{SessionId}").EnumerateFiles();
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
            return JsonSerializer.Deserialize<Session>(jsonString, options);
        }

        public static Session FromId(Guid sessionId)
        {
            return FromPath(GetSessionFilePathFromId(sessionId));
        }

        public static string GetSessionFilePathFromId(Guid sessionId)
        {
            return $"/{SESSION_ROOT_DIR}/{sessionId}/{SESSION_FILE_NAME}";
        }
    }
}