using System;
using System.Text;
using Microsoft.Extensions.Logging;

namespace DrDotnet;

public class Logger : ILogger
{
    public event Action<string>? MessageLogged;

    private StringBuilder _allLogs = new();

    // Todo: Needs refactoring
    public string GetAllLogs()
    {
        return _allLogs.ToString();
    }

    public IDisposable? BeginScope<TState>(TState state) where TState : notnull => default!;

    public bool IsEnabled(LogLevel logLevel) => true;

    public void Log<TState>(
        LogLevel logLevel,
        EventId eventId,
        TState state,
        Exception? exception,
        Func<TState, Exception?, string> formatter)
    {
        if (!IsEnabled(logLevel))
        {
            return;
        }

        var message = formatter(state, exception);
        
        var logMessage = $"[{logLevel}][{DateTime.Now}] {message}\n";
        if (exception != null)
        {
            logMessage += exception +"\n";
        }
        
        MessageLogged?.Invoke(logMessage);
        
        Console.Write(logMessage);
        _allLogs.Append(logMessage);
    }
}