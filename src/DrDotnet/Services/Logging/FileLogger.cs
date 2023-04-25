using System;
using System.IO;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace DrDotnet.Logging;

public class FileLoggerProvider : ILoggerProvider
{
    private readonly string _filePath;

    public FileLoggerProvider(string filePath)
    {
        _filePath = filePath;
    }

    public ILogger CreateLogger(string categoryName)
    {
        return new FileLogger(categoryName, _filePath);
    }

    public void Dispose()
    {
    }

    private class FileLogger : ILogger
    {
        private readonly string _filePath;
        private readonly string _categoryName;

        public FileLogger(string categoryName, string filePath)
        {
            _categoryName = categoryName;
            _filePath = filePath;
        }

        public IDisposable? BeginScope<TState>(TState state) where TState : notnull
        {
            return null;
        }

        public bool IsEnabled(LogLevel logLevel)
        {
            return true;
        }

        public void Log<TState>(LogLevel logLevel, EventId eventId, TState state, Exception? exception, Func<TState, Exception?, string> formatter)
        {
            if (!IsEnabled(logLevel))
            {
                return;
            }

            string logMessage = formatter(state, exception);
            string formattedLog = $"{DateTime.UtcNow:yyyy-MM-ddTHH:mm:ss.ffffZ} [{logLevel}] {_categoryName}: {logMessage}";

            if (exception != null)
            {
                formattedLog += Environment.NewLine + exception.ToString();
            }

            try
            {
                // Can be made more performant by using a stream probably
                File.AppendAllText(_filePath, formattedLog + Environment.NewLine);
            }
            catch (Exception) { }
        }
    }
}

public static class FileLoggerExtensions
{
    public static ILoggingBuilder AddFileLogger(this ILoggingBuilder builder, string filePath)
    {
        builder.Services.AddSingleton<ILoggerProvider>(new FileLoggerProvider(filePath));
        return builder;
    }
}