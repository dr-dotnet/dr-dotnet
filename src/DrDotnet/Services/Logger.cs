using System;
using System.Text;

namespace DrDotnet
{
    public class Logger : ILogger
    {
        public event Action<string> MessageLogged;

        private StringBuilder _allLogs = new StringBuilder();

        public string GetAllLogs()
        {
            return _allLogs.ToString();
        }

        public void Log(string message)
        {
            var logMessage = $"[{DateTime.Now}] {message}\n";
            MessageLogged?.Invoke(logMessage);
            Console.Write(logMessage);

            _allLogs.Append(logMessage);
        }
    }
}