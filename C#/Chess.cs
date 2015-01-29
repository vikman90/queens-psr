/*******************************************************************************
 * N queens problem (C# version)
 * Class Chess
 *
 * Copyleft 2013 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * February 8, 2013
 * 
 ******************************************************************************/

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
            IEnumerable<int> completeSet = Enumerable.Range(0, size);

            this.size = size;
            this.nSteps = 0;
            this.board = new HashSet<int>[size];

            for (int i = 0; i < size; i++)
                this.board[i] = new HashSet<int>(completeSet);

            this.discardedPairs = new Stack<int[]>();
            this.discardedCount = new Stack<int>();
        }

        //----------------------------------------------------------------------
        // Solve queens problem

        public bool Solve()
        {
            int index = SelectIndex();
            HashSet<int> currentSet;

            if (index == -1)
                return true;

            currentSet = board[index];
            board[index] = new HashSet<int>(currentSet);

            foreach (int value in currentSet) 
            {
                if (!Assign(index, value))
                {
                    board[index] = new HashSet<int>(currentSet);
                    continue;
                }

                if (Solve())
                    return true;

                RestoreLast();
                board[index] = new HashSet<int>(currentSet);
            }

            return false;
        }

        //----------------------------------------------------------------------
        // Representation of the chessboard

        public override string ToString()
        {
            StringBuilder builder = new StringBuilder();

            for (int i = 0; i < size; i++)
                builder.Append("Fila " + (i + 1) + " - columna " + (board[i].Single() + 1) + "\n");

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
        // Assign a value to a row and propagate constraints

        private bool Assign(int index, int value)
        {
            board[index].Clear();
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

            board[index].Add(value);
            return true;
        }

        //----------------------------------------------------------------------
        // Discard candidate values (constraints propagation)

        private bool Discard(int index, int value)
        {
            if (!board[index].Remove(value))
                return true;

            discardedPairs.Push(new int[] {index, value});
            discardedCount.Push(discardedCount.Pop() + 1);

            if (board[index].Count == 0)
                return false;

            if (board[index].Count == 1)
            {
                value = board[index].Single();

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
                board[pair[0]].Add(pair[1]);
            }
        }

        //----------------------------------------------------------------------
        // Get index of a unsolved row (minimum remaining values heuristics)

        private int SelectIndex()
        {
            int curCount, minCount = size + 1;
            int index = -1;

            for (int i = 0; i < size; i++)
            {
                curCount = board[i].Count;

                if (curCount > 1 && curCount < minCount)
                {
                    index = i;
                    minCount = curCount;
                }
            }

            return index;
        }

        //----------------------------------------------------------------------

        readonly int size;                      // Number of queens
        private long nSteps;                    // Number of calls to Assign()
        private HashSet<int>[] board;           // Queens' positions (set of candidate positions)
        private Stack<int[]> discardedPairs;    // Discarded candidates (index-value)
        private Stack<int> discardedCount;      // Number of discards in the last assignation
    }
}
