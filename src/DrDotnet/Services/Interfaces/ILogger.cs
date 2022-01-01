using System;

namespace DrDotnet
{
    public interface ILogger
    {
        public event Action<string> MessageLogged;

        public void Log(string message);

        public string GetAllLogs();
    }
}