﻿using System;
using System.Numerics;
using System.Runtime.CompilerServices;
using System.Threading.Tasks;

namespace DrDotnet.Tests.Simulations;

public class FibonacciSimulation : IDisposable
{
    private volatile bool _disposed = false;

    public FibonacciSimulation()
    {
        _ = Task.Run(() =>
        {
            while (!_disposed)
            {
                Calculate(1000);
            }
        });
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private long Calculate(int len)
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

public class FibonacciGeneric<T> : IDisposable
{
    private volatile bool _disposed = false;

    public FibonacciGeneric()
    {
        _ = Task.Run(() =>
        {
            while (!_disposed)
            {
                Calculate(nameof(T).Length * 100);
            }
        });
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private long Calculate(int len)
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