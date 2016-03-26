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
	int milliseconds;
	int n = 0;
	bool testing = false;

	for (int i = 1; i < argc; i++) {
		if (!strcmp(argv[i], "-test"))
			testing = true;

		else
			n = atoi(argv[i]);
	}

	while (n < 4) {
		cout << "Indique cantidad de reinas: ";
		cin >> n;
	}

	chess = new Chess(n);
	ticks = clock();
	chess->solve();
	milliseconds = (int)((double)(clock() - ticks) / CLOCKS_PER_SEC * 1000);

	if (testing) {
		cout << chess->getSteps() << "\t" << chess->getDiscards() << "\t" << milliseconds << endl;
	}
	else {
		cout << *chess << endl;
		cout << "Resuelto en " << chess->getSteps() << " pasos. ";
		cout << "Tiempo: " << milliseconds << " ms.\n";
	}

	delete chess;
	return EXIT_SUCCESS;
}