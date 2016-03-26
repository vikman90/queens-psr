#!/usr/bin/python3

################################################################################
# N queens problem (Python version)
# Class Chess
#
# Copyleft 2013 Vikman - All rights revoked.
# vikman90.blogspot.com - vmfdez90@gmail.com
# February 8, 2013
#
# Syntax: python queens.py [-test] [N]
# 
################################################################################

from chess import Chess
from time import clock
from sys import argv

if __name__ == "__main__":
    n = 0
    testing = False

    for arg in argv:
        if arg == "-test":
            testing = True
        else:
            try:
                n = int(arg)
            except ValueError:
                n = 0

    while n < 4:
        try:
            n = int(input("Indique cantidad de reinas: "))
        except ValueError:
            n = 0
            
    chess = Chess(n)
    time = clock()
    chess.solve()
    time = int((clock() - time) * 1000)

    if testing:
        print(str(chess.steps()) + "\t" + str(time));
    else:
        print("Resuelto en", chess.steps(), "pasos. Tiempo:", time, "ms.")
