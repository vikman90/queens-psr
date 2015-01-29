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

class Chess:
    def __init__(self, size):
        '''Constructor'''
        
        self._size = size               # Number of queens
        self._queens = []               # Calls to assign()
        self._nsteps = 0                # Queens' positions (candidates)
        
        for i in range(size):
            self._queens.append(set(range(size)))

        self._stackDiscarded = list()   # Discarded candidates (index-value)
        self._stackCount = list()       # Discards in the last assignation

    def solve(self):
        '''Solve queens problem'''
        
        index = self.selectIndex()

        if index == -1:
            return True

        row = self._queens[index]
        self._queens[index] = copy(row)

        for value in row:
            if not self.assign(index, value):
                self._queens[index] = copy(row)
                continue

            if self.solve():
                return True

            self.restoreLast()
            self._queens[index] = copy(row)

        return False   

    def selectIndex(self):
        '''Get index of a unsolved row (minimum remaining values heuristics)'''
        minSize = self._size + 1
        index = -1

        for i in range(self._size):
            curSize = len(self._queens[i])
            
            if curSize > 1 and curSize < minSize:
                index = i
                minSize = curSize

        return index

    def assign(self, index, value):
        '''Assign a value to a row and propagate constraints'''
        
        self._queens[index].clear()
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

        self._queens[index].add(value)
        return True        

    def discard(self, index, value):
        '''Discard candidate values (constraints propagation)'''
        
        row = self._queens[index]
        
        if not value in row:
            return True

        row.discard(value)
        self._stackDiscarded.append([index, value])
        self._stackCount.append(self._stackCount.pop() + 1)

        if len(row) == 0:
            return False

        if len(row) == 1:
            value = row.pop()
            row.add(value)

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
            self._queens[pair[0]].add(pair[1])

    def steps(self):
        '''Get number of total tries of assignation'''
        
        return self._nsteps
    
    def __str__(self):
        '''Representation of the chessboard'''
        
        string = str()
        
        for i in range(self._size):
            value = self._queens[i].pop()
            self._queens[i].add(value)
            string += "Reina " + str(i + 1) + ": " + str(value + 1) + "\n"

        return string[:-1]
