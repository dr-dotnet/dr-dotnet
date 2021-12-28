using System;
using System.Linq;
using System.Threading;

namespace Fibonacci
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Start");

            var myType = new MyType();

            long sum = 0;

            //Enumerable.Range(0, 2 /*Environment.ProcessorCount*/).Select((i) =>
            //{
            //    return ThreadPool.QueueUserWorkItem((_) =>
            //    {
                    while (true)
                    {
                        unchecked
                        {
                            Interlocked.Add(ref sum, myType.Fibonacci(1000 * 0));
                            try
                            {
                                throw new ArgumentNullException("test");
                            } catch
                            {

                            }
                        }
                    }
            //    });
            //}).ToArray();

            Console.WriteLine("Press key to exit");
            Console.Read();
        }
    }

    public class MyType
    {
        public int Fibonacci(int nth)
        {
            int a = 0, b = 1, c = 0;
            for (int i = 2; i < nth; i++)
            {
                c = a + b;
                a = b;
                b = c;
            }
            return c;
        }
    }
}
