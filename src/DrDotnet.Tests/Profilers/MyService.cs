﻿using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading;

namespace DrDotnet.Tests.Profilers;

public class MyService : IDisposable
{
    public readonly int _allocatedObjectsPerSecond = 1_000_000;
    public readonly int _maxAliveObjects = 100_000;

    private readonly Queue<string> _queue = new Queue<string>();

    private volatile bool _disposed = false;

    public MyService(int allocatedObjectsPerSecond, int maxAliveObjects)
    {
        _allocatedObjectsPerSecond = allocatedObjectsPerSecond;
        _maxAliveObjects = maxAliveObjects;

        ThreadPool.QueueUserWorkItem((_) =>
        {
            Span<char> strspan = stackalloc char[50];

            Stopwatch sw = Stopwatch.StartNew();

            int i = 0;

            while (!_disposed)
            {
                if (sw.Elapsed.TotalSeconds < i * 1d / _allocatedObjectsPerSecond)
                {
                    Thread.Sleep(10);
                }

                for (int j = 0; j < strspan.Length; j++)
                {
                    strspan[j] = (char)Random.Shared.Next(48, 122);
                }

                _queue.Enqueue(new string(strspan));

                if (_queue.Count > _maxAliveObjects)
                {
                    _queue.Dequeue();
                }

                i++;
            }
        });
    }

    public void Dispose()
    {
        _disposed = true;
    }
}