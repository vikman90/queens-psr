/*******************************************************************************
 * N queens problem (C++ version)
 * Main file
 *
 * Copyleft 2013 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * February 8, 2013
 *
 * Syntax: ./queens [-test] [N]
 *
 ******************************************************************************/

#include <iostream>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include "chess.h"

using namespace std;

int main(int argc, char **argv) {
	Chess *chess;
	clock_t ticks;
	long microseconds;
	int n = 0;
	bool testing = false;

	for (int i = 1; i < argc; i++) {
		if (!strcmp(argv[i], "-test"))
			testing = true;

		else
			n = atoi(argv[i]);
	}

	while (n < 4) {
		cout << "Enter number of queens: ";
		cin >> n;
	}

	chess = new Chess(n);
	ticks = clock();
	chess->solve();
	microseconds = (long)((double)(clock() - ticks) / CLOCKS_PER_SEC * 1000000);

	if (testing) {
		cout << chess->getSteps() << "\t" << chess->getDiscards() << "\t" << microseconds << endl;
	}
	else {
		cout << *chess << endl;
		cerr << "Steps:       " << chess->getSteps() << endl;
		cerr << "Discards:    " << chess->getDiscards() << endl;
		cerr << "Time:        " << microseconds / 1000.0 << " ms." << endl;
		cerr << "Performance: " << chess->getSteps() * 1000 / microseconds << " steps/ms." << endl;
		cerr << "             " << chess->getDiscards() * 1000 / microseconds << " discards/ms." << endl;
	}

	delete chess;
	return EXIT_SUCCESS;
}
