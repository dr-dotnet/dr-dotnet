using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;

namespace DrDotnet
{
    public class SessionDiscovery : ISessionDiscovery
    {
        private ILogger _logger;

        public SessionDiscovery(ILogger logger)
        {
            _logger = logger;
        }

        public List<Session> GetSessions()
        {
            var sessions = new List<Session>();

            var options = new JsonSerializerOptions
            {
                PropertyNameCaseInsensitive = true
            };

            string[] subdirectoryEntries = Directory.GetDirectories("/dr-dotnet/");
            foreach (string subdirectory in subdirectoryEntries)
            {
                string sessionFilePath = Path.Combine(subdirectory, Session.SESSION_FILE_NAME);
                try
                {
                    sessions.Add(Session.FromPath(sessionFilePath));
                }
                catch(Exception ex)
                {
                    _logger.Log(ex.ToString());
                }
            }

            return sessions;
        }
    }
}