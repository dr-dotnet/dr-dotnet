using System;
using System.Threading;

namespace DrDotnet.Tests.Simulations;

public class LockingSimulation : IDisposable
{
    private volatile bool _disposed = false;

    public LockingSimulation(object lockObject)
    {
        ThreadPool.QueueUserWorkItem(_ =>
        {
            while (!_disposed)
            {
                lock (lockObject)
                {
                    Thread.Sleep(Random.Shared.Next(10, 500));
                }
                Thread.Sleep(Random.Shared.Next(10, 500));
            }
        });
    }

    public void Dispose()
    {
        _disposed = true;
    }
}