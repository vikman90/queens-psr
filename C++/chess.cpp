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

using namespace std;



static int* newPair(int a, int b)
{
	int *pair = new int[2];
	pair[0] = a;
	pair[1] = b;
	return pair;
}

//------------------------------------------------------------------------------
// Constructor

Chess::Chess(int _size) : size(_size), nSteps(0), queens(vector< set<int> >(_size))
{
	set<int> completeSet;

	for (int i = 0; i < _size; i++)
		completeSet.insert(i);

	for (int i = 0; i < _size; i++)
		queens[i] = completeSet;
}

//------------------------------------------------------------------------------
// Solve queens problem

bool Chess::solve()
{
	int index = selectIndex();
	set<int> currentSet;
	set<int>::iterator itValue;

	if (index == -1)
		return true;

	currentSet = queens[index];
	
	for (itValue = currentSet.begin(); itValue != currentSet.end(); ++itValue) {
		int value = *itValue;

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
		if (chess.queens[i].size() != 1)
			stream << "Reina " << i + 1 << " no resuelta.\n";

		else {
			int value = *chess.queens[i].begin();
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

	discardedPairs.push(newPair(index, value));
	discardedCount.top() += 1;

	if (queens[index].size() == 0)
		return false;

	if (queens[index].size() == 1) {
		value = *queens[index].begin();

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
	int *pair;
	int n = discardedCount.top();

	for (int i = 0; i < n; i++) {
		pair = discardedPairs.top();
		queens[pair[0]].insert(pair[1]);
		discardedPairs.pop();
		delete [] pair;
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
		curSize = queens[i].size();

		if (curSize > 1 && curSize < minSize) {
			index = i;
			minSize = curSize;
		}
	}

	return index;
}
