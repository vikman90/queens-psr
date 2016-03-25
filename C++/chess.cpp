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
#include <cstdlib>
#include <cstring>
#include <iostream>

using namespace std;

//------------------------------------------------------------------------------
// Constructor

Chess::Chess(int _size) : size(_size), nSteps(0), queens(vector< char* >(_size)), queensCount(vector<int>(_size, _size))
{
	char complete[_size];

	for (int i = 0; i < _size; i++)
        complete[i] = 1;

	for (int i = 0; i < _size; i++) {
		queens[i] = (char*)malloc(_size);
        memcpy(queens[i], complete, _size);
    }
}

//------------------------------------------------------------------------------
// Destructor

Chess::~Chess()
{
    for (int i = 0; i < size; i++)
        free(queens[i]);
}

//------------------------------------------------------------------------------
// Solve queens problem

bool Chess::solve()
{
	int index = selectIndex();
	char currentSet[size];
    int values[size];
    int nvalues;

	if (index == -1)
		return true;

	memcpy(currentSet, queens[index], size);
    nvalues = selectValues(currentSet, values);

	for (int i = 0; i < nvalues; i++) {
		int value = values[i];

		if (!assign(index, value)) {
			memcpy(queens[index], currentSet, size);
            queensCount[index] = nvalues;
			continue;
		}

		if (solve())
			return true;

		restoreLast();
        memcpy(queens[index], currentSet, size);
        queensCount[index] = nvalues;
	}

	return false;
}

//------------------------------------------------------------------------------
// Representation of the chessboard

ostream& operator<< (ostream &stream, const Chess &chess)
{
	for (int i = 0; i < chess.size; i++) {
		if (chess.queensCount[i] != 1)
			stream << "Reina " << i + 1 << " no resuelta.\n";

		else {
			int value = chess.getValue(chess.queens[i]);
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

	memset(queens[index], 0, size);
    queensCount[index] = 0;
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

	queens[index][value] = 1;
    queensCount[index] = 1;
	return true;
}

//------------------------------------------------------------------------------
// Discard candidate values (constraints propagation)

bool Chess::discard(int index, int value)
{
	int diag1, diag2;

    if (value < 0 || value >= size || !queens[index][value])
        return true;

    queens[index][value] = 0;
    queensCount[index]--;
	discardedPairs.push(Pair {index, value});
	discardedCount.top() += 1;

	if (queensCount[index] == 0)
		return false;

	if (queensCount[index] == 1) {
		value = getValue(queens[index]);

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

        if (!queens[pair.index][pair.value]) {
            queens[pair.index][pair.value] = 1;
            queensCount[pair.index]++;
        }

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
		curSize = queensCount[i];

		if (curSize > 1 && curSize < minSize) {
			index = i;
			minSize = curSize;
		}
	}

	return index;
}

//------------------------------------------------------------------------------
// Select all available indices from a row
int Chess::selectValues(const char *array, int *values)
{
    int nvalues = 0;

    for (int i = 0; i < size; i++) {
        if (array[i])
            values[nvalues++] = i;
    }

    return nvalues;
}

//------------------------------------------------------------------------------
// Get the __only__ value that is set in the array
int Chess::getValue(const char *array) const {
    for (int i = 0; i < size; i++)
        if (array[i])
            return i;

    cerr << "Error inesperado: array vacio.\n";
    exit(EXIT_FAILURE);
    return -1;
}
