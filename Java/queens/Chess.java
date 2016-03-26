/**
 * *****************************************************************************
 * N queens problem (Java version) Class Chess
 *
 * Copyleft 2013 Vikman - All rights revoked. vikman90.blogspot.com -
 * vmfdez90@gmail.com February 8, 2013
 *
 *****************************************************************************
 */

package queens;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Random;

public class Chess {

    //--------------------------------------------------------------------------
    // Constructor

    public Chess(int size) {
        byte[] complete = new byte[size];

        this.size = size;
        this.nSteps = 0;
        this.queens = new ArrayList<byte[]>(size);
        this.queensCount = new ArrayList<Integer>(size);

        Arrays.fill(complete, (byte)1);

        for (int i = 0; i < size; i++) {
            this.queens.add((byte[])complete.clone());
            this.queensCount.add(size);
        }

        this.discardedPairs = new ArrayDeque<int[]>();
        this.discardedCount = new ArrayDeque<Integer>();
        this.random = new Random();
    }

    //--------------------------------------------------------------------------
    // Solve queens problem

    public boolean solve() {
        int index = selectIndex();
        byte[] currentSet;
        int[] values = new int[size];
        int nvalues;

        if (index == -1)
            return true;

        currentSet = (byte[])queens.get(index).clone();
        nvalues = selectValues(currentSet, values);

        for (int i = 0; i < nvalues; i++) {
            int value = values[i];

            if (!assign(index, value)) {
                System.arraycopy(currentSet, 0, queens.get(index), 0, size);
                queensCount.set(index, nvalues);
                continue;
            }

            if (solve()) {
                return true;
            }

            restoreLast();
            System.arraycopy(currentSet, 0, queens.get(index), 0, size);
            queensCount.set(index, nvalues);
        }

        return false;
    }

    //--------------------------------------------------------------------------
    // Representation of the chessboard

    public String toString() {
        StringBuilder builder = new StringBuilder();

        for (int i = 0; i < size; i++) {
            if (queensCount.get(i) != 1)
                builder.append("Fila " + (i + 1) + " no resuelta.\n");
            else    {
                int value = getValue(i);
                builder.append("Reina " + (i + 1) + ": casilla " + (value + 1) + "\n");
            }
        }

        return builder.toString();
    }

    //--------------------------------------------------------------------------
    // Get number of total tries of assignation

    public long getSteps() {
        return nSteps;
    }

    //--------------------------------------------------------------------------
    // Assign a value to a row and propagate constraints

    private boolean assign(int index, int value) {
        nSteps++;
        Arrays.fill(queens.get(index), (byte)0);
        queensCount.set(index, 0);
        discardedCount.push(0);

        for (int i = 0; i < size; i++) {
            if (i == index) {
                continue;
            }

            int diag1 = value + (index - i);
            int diag2 = value - (index - i);

            if (!(discard(i, value) && discard(i, diag1) && discard(i, diag2))) {
                restoreLast();
                return false;
            }
        }

        queens.get(index)[value] = 1;
        queensCount.set(index, 1);
        return true;
    }

    //--------------------------------------------------------------------------
    // Discard candidate values (constraints propagation)

    private boolean discard(int index, int value) {
        int count;

        if (value < 0 || value >= size || queens.get(index)[value] == 0)
            return true;

        queens.get(index)[value] = 0;
        count = queensCount.get(index) - 1;
        queensCount.set(index, count);
        discardedPairs.push(new int[] {index, value});
        discardedCount.push(discardedCount.pop() + 1);

        if (count == 0)
            return false;

        if (count == 1) {
            value = getValue(index);

            for (int i = 0; i < size; i++) {
                if (i == index) {
                    continue;
                }

                int diag1 = value + (index - i);
                int diag2 = value - (index - i);

                if (!(discard(i, value) && discard(i, diag1) && discard(i, diag2))) {
                    return false;
                }
            }
        }

        return true;
    }

    //--------------------------------------------------------------------------
    // Undo last assignation (restore constraints)

    private void restoreLast() {
        int n = discardedCount.pop();

        for (int i = 0; i < n; i++) {
            int[] pair = discardedPairs.pop();
            byte[] array = queens.get(pair[0]);

            if (array[pair[1]] == 0) {
                array[pair[1]] = 1;
                queensCount.set(pair[0], queensCount.get(pair[0]) + 1);
            }
        }
    }

    //--------------------------------------------------------------------------
    // Get index of a unsolved row (minimum remaining values heuristics)

    private int selectIndex() {
        int curSize, minSize = size + 1;
        int index = -1;

        for (int i = 0; i < size; i++) {
            curSize = queensCount.get(i);

            if (curSize > 1 && curSize < minSize) {
                index = i;
                minSize = curSize;
            }
        }

        return index;
    }

    //--------------------------------------------------------------------------
    // Select all available indices from a row

    private int selectValues(byte[] array, int[] values) {
        int nvalues = 0;
        int index;
        int offset = random.nextInt(size);

        for (int i = 0; i < size; i++) {
            index = (offset + i) % size;

            if (array[index] != 0)
                values[nvalues++] = index;
        }

        return nvalues;
    }

    //--------------------------------------------------------------------------
    // Get the __only__ value that is set in the indexed row

    private int getValue(int index) {
        for (int i = 0; i < size; i++)
            if (queens.get(index)[i] != 0)
                return i;

        return -1;
    }

    //--------------------------------------------------------------------------

    private final int size;						// Number of queens
    private long nSteps;						// Number of calls to assign()
    private ArrayList<byte[]> queens;	        // Queens' positions (set of candidate positions)
    private ArrayList<Integer> queensCount;     // Number of available values
    private ArrayDeque<int[]> discardedPairs;	// Discarded candidates (index-value)
    private ArrayDeque<Integer> discardedCount;	// Number of discards in the last assignation
    private Random random;                      // Generator of random numbers
}
