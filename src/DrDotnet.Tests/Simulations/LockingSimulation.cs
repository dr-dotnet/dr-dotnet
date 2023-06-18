using System;
using System.Threading.Tasks;

namespace DrDotnet.Tests.Simulations;

public class FibonacciSimulation : IDisposable
{
    private volatile bool _disposed = false;

    public FibonacciSimulation()
    {
        var task = Task.Run(() =>
        {
            while (!_disposed)
            {
                Calculate(1000);
            }
        });
    }

    public long Calculate(int len)
    {
        long a = 0, b = 1, c = 0;   
        for (int i = 2; i < len; i++)  
        {  
            c = a + b;  
            a = b;  
            b = c;  
        }

        return c;
    }

    public void Dispose()
    {
        _disposed = true;
    }
}