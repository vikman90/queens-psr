/*******************************************************************************
 * N queens problem (C++ version)
 * Class Chess
 *
 * Copyleft 2013 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * February 8, 2013
 *
 ******************************************************************************/

#include "chess.h"

using std::ostream;
using std::endl;

//------------------------------------------------------------------------------
// Solve queens problem

bool Chess::solve()
{
	int index = selectIndex();
    int values[size];
    int nvalues;

	if (index == -1)
		return true;

    Set currentSet = queens[index];
    nvalues = currentSet.selectValues(values);

	for (int i = 0; i < nvalues; i++) {
		int value = values[i];

		if (!assign(index, value)) {
			queens[index] = currentSet;
			continue;
		}

		if (solve())
			return true;

		restoreLast();
        queens[index] = currentSet;
	}

	return false;
}

//------------------------------------------------------------------------------
// Representation of the chessboard

ostream& operator<< (ostream &stream, const Chess &chess)
{
	for (int i = 0; i < chess.size; i++) {
		if (chess.queens[i].count() != 1)
			stream << "Reina " << i + 1 << " no resuelta.\n";

		else {
			int value = chess.queens[i].value();
			stream << "Reina " << (i + 1) << ": casilla " << (value + 1) << endl;
		}
	}

	return stream;
}

//------------------------------------------------------------------------------
// Assign a value to a row and propagate constraints

bool Chess::assign(int index, int value)
{
	int diag1, diag2;
	nSteps++;

	queens[index].clear();
	discardedCount.push(0);

	for (int i = 0; i < size; i++) {
		if (i == index)
			continue;

		diag1 = value + (index - i);
		diag2 = value - (index - i);

		if (!(discard(i, value) && discard(i, diag1) && discard(i, diag2))) {
			restoreLast();
			return false;
		}
	}

	queens[index].insert(value);
	return true;
}

//------------------------------------------------------------------------------
// Discard candidate values (constraints propagation)

bool Chess::discard(int index, int value)
{
	int diag1, diag2;

    if (queens[index].erase(value) == 0)
        return true;

	discardedPairs.push(Pair {index, value});
	discardedCount.top() += 1;

	if (queens[index].count() == 0)
		return false;

	if (queens[index].count() == 1) {
		value = queens[index].value();

		for (int i = 0; i < size; i++) {
			if (i == index)
				continue;

			diag1 = value + (index - i);
			diag2 = value - (index - i);

			if (!(discard(i, value) && discard(i, diag1) && discard(i, diag2)))
				return false;
		}
	}

	return true;
}

//------------------------------------------------------------------------------
// Undo last assignation (restore constraints)

void Chess::restoreLast()
{
    Pair pair;
	int n = discardedCount.top();

	for (int i = 0; i < n; i++) {
		pair = discardedPairs.top();
        queens[pair.index].insert(pair.value);
		discardedPairs.pop();
	}

	discardedCount.pop();
}

//------------------------------------------------------------------------------
// Get index of a unsolved row (minimum remaining values heuristics)

int Chess::selectIndex()
{
	int curSize, minSize = size + 1;
	int index = -1;

	for (int i = 0; i < size; i++) {
		curSize = queens[i].count();

		if (curSize > 1 && curSize < minSize) {
			index = i;
			minSize = curSize;
		}
	}

	return index;
}
