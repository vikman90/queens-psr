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
import java.util.HashSet;

public class Chess {

    //--------------------------------------------------------------------------
    // Constructor
    @SuppressWarnings("unchecked")
    public Chess(int size) {
        HashSet<Integer> completeSet = new HashSet<Integer>(size);

        this.size = size;
        this.nSteps = 0;
        this.board = new ArrayList<HashSet<Integer>>(size);

        for (int i = 0; i < size; i++) {
            completeSet.add(i);
        }

        for (int i = 0; i < size; i++) {
            this.board.add((HashSet<Integer>) completeSet.clone());
        }

        this.discardedPairs = new ArrayDeque<int[]>();
        this.discardedCount = new ArrayDeque<Integer>();
    }

    //--------------------------------------------------------------------------
    // Solve queens problem
    @SuppressWarnings("unchecked")
    public boolean solve() {
        int index = selectIndex();
        HashSet<Integer> currentSet;

        if (index == -1) {
            return true;
        }

        currentSet = board.get(index);
        board.set(index, (HashSet<Integer>) currentSet.clone());

        for (int value : currentSet) {
            if (!assign(index, value)) {
                board.set(index, (HashSet<Integer>) currentSet.clone());
                continue;
            }

            if (solve()) {
                return true;
            }

            restoreLast();
            board.set(index, (HashSet<Integer>) currentSet.clone());
        }

        return false;
    }

    //--------------------------------------------------------------------------
    // Representation of the chessboard
    public String toString() {
        StringBuilder builder = new StringBuilder();

        for (int i = 0; i < size; i++) {
            if (board.get(i).size() != 1) {
                System.err.println("Error: Fila " + (i + 1) + " no resuelta.");
                return "";
            }

            int value = board.get(i).iterator().next() + 1;
            builder.append("Reina " + (i + 1) + ": casilla " + (value + 1) + "\n");
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
        HashSet<Integer> set = board.get(index);

        assert set.size() > 0;
        assert set.contains(value);
        set.clear();
        nSteps++;

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

        set.add(value);
        return true;
    }

    //--------------------------------------------------------------------------
    // Discard candidate values (constraints propagation)
    private boolean discard(int index, int value) {
        HashSet<Integer> set = board.get(index);

        if (!set.remove(value)) {
            return true;
        }

        discardedPairs.push(new int[]{index, value});
        discardedCount.push(discardedCount.pop() + 1);

        if (set.size() == 0) {
            return false;
        }

        if (set.size() == 1) {
            value = set.iterator().next();

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
            board.get(pair[0]).add(pair[1]);
        }
    }

    //--------------------------------------------------------------------------
    // Get index of a unsolved row (minimum remaining values heuristics)
    private int selectIndex() {
        int curSize, minSize = size + 1;
        int index = -1;

        for (int i = 0; i < size; i++) {
            curSize = board.get(i).size();

            try {
                assert curSize > 0;
            } catch (AssertionError error) {
                System.err.println("Error: fila " + i + " vacia.");
                throw error;
            }

            if (curSize > 1 && curSize < minSize) {
                index = i;
                minSize = curSize;
            }
        }

        return index;
    }
    //--------------------------------------------------------------------------
    private final int size;						// Number of queens
    private long nSteps;						// Number of calls to assign()
    private ArrayList<HashSet<Integer>> board;	// Queens' positions (set of candidate positions)
    private ArrayDeque<int[]> discardedPairs;	// Discarded candidates (index-value)
    private ArrayDeque<Integer> discardedCount;	// Number of discards in the last assignation
}
