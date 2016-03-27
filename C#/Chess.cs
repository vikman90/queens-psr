/*******************************************************************************
 * N queens problem (C# version)
 * Class Chess
 *
 * Copyleft 2013 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * February 8, 2013
 * 
 ******************************************************************************/

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Queens
{
    class Chess
    {
        //----------------------------------------------------------------------
        // Constructor

        public Chess(int size)
        {
            byte[] completeSet = Enumerable.Repeat((byte)1, size).ToArray();

            this.size = size;
            nSteps = 0;
            nDiscards = 0;
            queens = new byte[size][];
            queensCount = Enumerable.Repeat(size, size).ToArray();

            for (int i = 0; i < size; i++)
                this.queens[i] = (byte[])completeSet.Clone();

            discardedPairs = new Stack<int[]>();
            discardedCount = new Stack<int>();
            random = new Random();
        }

        //----------------------------------------------------------------------
        // Solve queens problem

        public bool Solve()
        {
            int index = SelectIndex();
            int nvalues;
            byte[] currentSet;
            int[] values = new int[size];

            if (index == -1)
                return true;

            currentSet = (byte[])queens[index].Clone();
            nvalues = SelectValues(index, values);

            for (int i = 0; i < nvalues; i++) 
            {
                int value = values[i];

                if (!Assign(index, value))
                {
                    queens[index] = (byte[])currentSet.Clone();
                    queensCount[index] = nvalues;
                    continue;
                }

                if (Solve())
                    return true;

                RestoreLast();
                queens[index] = (byte[])currentSet.Clone();
                queensCount[index] = nvalues;
            }

            return false;
        }

        //----------------------------------------------------------------------
        // Representation of the chessboard

        public override string ToString()
        {
            StringBuilder builder = new StringBuilder();

            for (int i = 0; i < size; i++)
                if (queensCount[i] != 1)
                    builder.Append("Reina " + (i + 1) + " no resuelta.\n");
                else
                    builder.Append("Reina " + (i + 1) + ": casilla " + (Array.IndexOf(queens[i], (byte)1) + 1) + "\n");

            return builder.ToString();
        }

        //----------------------------------------------------------------------
        // Get number of total tries of assignation

        public long Steps
        {
            get
            {
                return nSteps;
            }
        }

        //----------------------------------------------------------------------
        // Get number of total discards

        public long Discards
        {
            get
            {
                return nDiscards;
            }
        }

        //----------------------------------------------------------------------
        // Assign a value to a row and propagate constraints

        private bool Assign(int index, int value)
        {
            Array.Clear(queens[index], 0, size);
            queensCount[index] = 0;
            discardedCount.Push(0);
            nSteps++;

            for (int i = 0; i < size; i++)
            {
                if (i == index)
                    continue;

                int diag1 = value + (index - i);
                int diag2 = value - (index - i);

                if (!(Discard(i, value) && Discard(i, diag1) && Discard(i, diag2)))
                {
                    RestoreLast();
                    return false;
                }
            }

            queens[index][value] = 1;
            queensCount[index] = 1;
            return true;
        }

        //----------------------------------------------------------------------
        // Discard candidate values (constraints propagation)

        private bool Discard(int index, int value)
        {
            if (value < 0 || value >= size || queens[index][value] == 0)
                return true;

            nDiscards++;
            queens[index][value] = 0;
            queensCount[index]--;
            discardedPairs.Push(new int[] {index, value});
            discardedCount.Push(discardedCount.Pop() + 1);

            if (queensCount[index] == 0)
                return false;

            if (queensCount[index] == 1)
            {
                value = Array.IndexOf(queens[index], (byte)1);

                for (int i = 0; i < size; i++)
                {
                    if (i == index)
                        continue;

                    int diag1 = value + (index - i);
                    int diag2 = value - (index - i);

                    if (!(Discard(i, value) && Discard(i, diag1) && Discard(i, diag2)))
                        return false;
                }
            }

            return true;
        }

        //----------------------------------------------------------------------
        // Undo last assignation (restore constraints)

        private void RestoreLast()
        {
            int n = discardedCount.Pop();

            for (int i = 0; i < n; i++)
            {
                int[] pair = discardedPairs.Pop();
                
                if (queens[pair[0]][pair[1]] == 0)
                {
                    queens[pair[0]][pair[1]] = 1;
                    queensCount[pair[0]]++;
                }
            }
        }

        //----------------------------------------------------------------------
        // Get index of a unsolved row (minimum remaining values heuristics)

        private int SelectIndex()
        {
            int minCount = size + 1;
            int index = -1;

            for (int i = 0; i < size; i++)
            {
                if (queensCount[i] > 1 && queensCount[i] < minCount)
                {
                    index = i;
                    minCount = queensCount[i];
                }
            }

            return index;
        }

        //----------------------------------------------------------------------
        // Select all available indices from a row

        private int SelectValues(int index, int[] values)
        {
            int nvalues = 0;
            int offset = random.Next(size);

            for (int i = 0; i < size; i++)
            {
                int value = (offset + i) % size;

                if (queens[index][value] != 0)
                    values[nvalues++] = value;
            }

            return nvalues;
        }

        //----------------------------------------------------------------------

        readonly int size;                      // Number of queens
        private long nSteps;                    // Number of calls to Assign()
        private long nDiscards;                 // Number of calls to Discard()
        private byte[][] queens;                // Queens' positions (set of candidate positions)
        private int[] queensCount;              // Number of available values
        private Stack<int[]> discardedPairs;    // Discarded candidates (index-value)
        private Stack<int> discardedCount;      // Number of discards in the last assignation
        private Random random;                  // Generator of random numbers
    }
}
