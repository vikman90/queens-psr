################################################################################
# N queens problem (Python version)
# Class Chess
#
# Copyleft 2013 Vikman - All rights revoked.
# vikman90.blogspot.com - vmfdez90@gmail.com
# February 8, 2013
#
################################################################################

from copy import copy
from time import clock
from sys import argv
from random import seed, randint

seed()

class Chess:
    def __init__(self, size):
        '''Constructor'''

        self._size = size               # Number of queens
        # Queens' positions (candidates)
        self._queens = [[1 for j in range(size)] for i in range(size)]
        # Number of available values in each row
        self._queensCount = [size for i in range(size)]
        self._nsteps = 0                # Calls to assign()
        self._ndiscards = 0             # Calls to discard()
        self._stackDiscarded = list()   # Discarded candidates (index-value)
        self._stackCount = list()       # Discards in the last assignation

    def solve(self):
        '''Solve queens problem'''

        index = self.selectIndex()

        if index == -1:
            return True

        row = copy(self._queens[index])
        values = self.selectValues(row)

        for value in values:
            if not self.assign(index, value):
                self._queens[index] = copy(row)
                self._queensCount[index] = len(values)
                continue

            if self.solve():
                return True

            self.restoreLast()
            self._queens[index] = copy(row)
            self._queensCount[index] = len(values)

        return False

    def selectIndex(self):
        '''Get index of a unsolved row (minimum remaining values heuristics)'''
        minSize = self._size + 1
        index = -1

        for i in range(self._size):
            curSize = self._queensCount[i]

            if curSize > 1 and curSize < minSize:
                index = i
                minSize = curSize

        return index

    def assign(self, index, value):
        '''Assign a value to a row and propagate constraints'''

        self._queens[index] = [0 for i in range(self._size)]
        self._queensCount[index] = 0
        self._stackCount.append(0)
        self._nsteps += 1

        for i in range(self._size):
            if i == index:
                continue

            diag1 = value + (index - i)
            diag2 = value - (index - i)

            if not (self.discard(i, value) and self.discard(i, diag1)
                    and self.discard(i, diag2)):
                self.restoreLast()
                return False

        self._queens[index][value] = 1
        self._queensCount[index] = 1
        return True

    def discard(self, index, value):
        '''Discard candidate values (constraints propagation)'''

        if value < 0 or value >= self._size or not self._queens[index][value]:
            return True

        self._ndiscards += 1
        self._queens[index][value] = 0
        self._queensCount[index] -= 1
        self._stackDiscarded.append([index, value])
        self._stackCount.append(self._stackCount.pop() + 1)

        if self._queensCount[index] == 0:
            return False

        if self._queensCount[index] == 1:
            value = self.getValue(index)

            for i in range(self._size):
                if i == index:
                    continue

                diag1 = value + (index - i)
                diag2 = value - (index - i)

                if not (self.discard(i, value) and self.discard(i, diag1)
                        and self.discard(i, diag2)):
                    return False

        return True

    def restoreLast(self):
        '''Undo last assignation (restore constraints)'''

        count = self._stackCount.pop()

        for i in range(count):
            pair = self._stackDiscarded.pop()

            if not self._queens[pair[0]][pair[1]]:
                self._queens[pair[0]][pair[1]] = 1
                self._queensCount[pair[0]] += 1

    def steps(self):
        '''Get number of total tries of assignation'''

        return self._nsteps

    def discards(self):
        '''Get number of total discards'''

        return self._ndiscards

    def selectValues(self, row):
        '''Select all available indices from a row'''

        values = []
        offset = randint(0, self._size - 1)

        for i in range(len(row)):
            index = (offset + i) % self._size

            if row[index]:
                values.append(index)

        return values

    def getValue(self, index):
        '''Get the only value that is set in the indexed row'''

        for i in range(self._size):
            if self._queens[index][i]:
                return i

        return -1

    def __str__(self):
        '''Representation of the chessboard'''

        string = str()

        for i in range(self._size):
            if self._queensCount[i] == 0:
                string += "Queen " + str(i + 1) + " not solved.\n"
            else:
                value = self._queens[i][0] + 1
                string += "Queen " + str(i + 1) + ": square " + str(value) + "\n"

        return string[:-1]
