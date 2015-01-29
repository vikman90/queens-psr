﻿/*******************************************************************************
 * N queens problem (C# version)
 * Main file
 *
 * Copyleft 2013 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * February 8, 2013
 *
 * Syntax: queens [-test] [N]
 * 
 ******************************************************************************/

using System;

namespace Queens
{
    class Program
    {
        static void Main(string[] args)
        {
            Chess chess;
            int n = 0;
            long time;
            bool testing = false;

            foreach (string arg in args) {
                if (arg == "-test")
                    testing = true;
                else
                    n = int.Parse(args[0]);
            }

            while (n < 4)
            {
                Console.Write("Introduzca cantidad de reinas: ");
                n = int.Parse(Console.ReadLine());
            }

            chess = new Chess(n);
            time = DateTime.Now.Ticks;
            chess.Solve();
            time = DateTime.Now.Ticks - time;

            if (testing)
                Console.WriteLine(chess.Steps + "\t" + time / 10000);
            else
            {
                Console.WriteLine(chess.ToString());
                Console.Write("Resuelto en " + chess.Steps + " pasos. ");
                Console.WriteLine("Tiempo: " + time / 10000 + " ms.");
            }
        }
    }
}
