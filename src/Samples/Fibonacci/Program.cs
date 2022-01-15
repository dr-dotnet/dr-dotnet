using System;
using System.Linq;
using System.Threading;

namespace Fibonacci
{
    class AllocObject
    {
        public static readonly int ArraySize = 100;
        int[] m_array = null;

        public AllocObject(bool poh)
        {
            if (poh)
            {
                m_array = GC.AllocateArray<int>(ArraySize, true);
            }
            else
            {
                m_array = new int[ArraySize];
            }
        }
    }

    class Program
    {
        /*
        public static ulong StaticValue;

        static void Main(string[] args)
        {
            Console.WriteLine("Start");

            long sum = 0;

            MyNode node = new MyNode();

            Enumerable.Range(0, 2 Environment.ProcessorCount).Select((i) =>
            {
                return ThreadPool.QueueUserWorkItem((_) =>
                {
                    while (true)
                    {
                        var myType = new MyType();

                        unchecked
                        {
                            node = node.ChildNode = new MyNode(); // Hold some memory that will survive over garbage collection

                            Interlocked.Add(ref sum, myType.Fibonacci(1000));
                            try
                            {
                                MyMethod1();
                            } catch
                            {

                            }
                        }
                    }
                });
            }).ToArray();

            Console.WriteLine("Press key to exit");
            Console.Read();
        }

        static void MyMethod1()
        {
            ThrowSomeException();
        }

        static void ThrowSomeException()
        {
            throw new ArgumentNullException("test");
        }
    */
        public static int RunTest(String[] args)
        {
            int numAllocators = 1024;
            int[] root1 = GC.AllocateArray<int>(AllocObject.ArraySize, true);
            int[] root2 = GC.AllocateArray<int>(AllocObject.ArraySize, true);
            AllocObject[] objs = new AllocObject[numAllocators];

            Random rand = new Random();
            int numPoh = 0;
            int numReg = 0;
            int i = 0;
            while (true)
            {
                i++;

                int pos = rand.Next(0, numAllocators);

                bool poh = rand.NextDouble() > 0.5;
                objs[pos] = new AllocObject(poh);

                if (poh)
                {
                    ++numPoh;
                }
                else
                {
                    ++numReg;
                }

                if (i % 1000 == 0)
                {
                    GC.Collect();
                    Console.WriteLine($"Did {i} iterations Allocated={GC.GetAllocatedBytesForCurrentThread()}");
                }


                int[] m_array = new int[100];
            }

            Console.WriteLine($"{numPoh} POH allocs and {numReg} normal allocs.");
            GC.KeepAlive(root1);
            GC.KeepAlive(root2);
            return 100;
        }

        public static int Main(string[] args)
        {
            return RunTest(args);
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

        ~MyType()
        {
            //Program.StaticValue++;
        }
    }

    public class MyNode
    {
        public int value1;
        public int value2;

        public MyNode ChildNode { get; set; }
    }
}
