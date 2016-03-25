/*******************************************************************************
 * N queens problem (C++ version)
 * Class Chess
 *
 * Copyleft 2013 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * February 8, 2013
 *
 ******************************************************************************/

#ifndef CHESS_H
#define CHESS_H

#include <ostream>
#include <vector>
#include <stack>

struct Pair
{
    int index;
    int value;
};

class Chess
{
public:

	// Constructor
	explicit Chess(int size);

	// Solve queens problem
	bool solve();

	// Representation of the chessboard
	friend std::ostream& operator<< (std::ostream &stream, const Chess &chess);

	// Get number of total tries of assignation
	inline long long getSteps()
	{
		return nSteps;
	}

private:

	// Assign a value to a row and propagate constraints
	bool assign(int index, int value);

	// Discard candidate values (constraints propagation)
	bool discard(int index, int value);

	// Undo last assignation (restore constraints)
	void restoreLast();

	// Get index of a unsolved row (minimum remaining values heuristics)
	int selectIndex();

    // Select all available indices from a row
    int selectValues(const int *array, int *values);

    // Get the __only__ value that is set in the array
    int getValue(const int *array) const;

	int size;							// Number of queens
	long long nSteps;					// Number of calls to Assign()
	std::vector< int* > queens;	        // Queens' positions (set of candidate positions)
    std::vector<int> queensCount;       // Number of available values
	std::stack<Pair> discardedPairs;    // Discarded candidates (index-value)
	std::stack<int> discardedCount;		// Number of discards in the last assignation
};

#endif
