using System;
using System.Threading;

namespace DrDotnet.Tests.Simulations;

public class PrimeSimulation : IDisposable
{
    private volatile bool _disposed = false;

    private int _currentPrime;

    public int CurrentPrime => _currentPrime;

    public PrimeSimulation()
    {
        _currentPrime = 2;
        int currentNumber = 3;
        
        ThreadPool.QueueUserWorkItem(_ =>
        {
            while (!_disposed)
            {
                if (IsPrime(currentNumber))
                    _currentPrime = currentNumber;

                currentNumber += 2; // Skip even numbers since they can't be prime
            }
        });
    }

    private static bool IsPrime(int num)
    {
        if (num < 2)
            return false;

        for (int i = 2; i * i <= num; i++)
        {
            if (num % i == 0)
                return false;
        }

        return true;
    }

    public void Dispose()
    {
        _disposed = true;
    }
}