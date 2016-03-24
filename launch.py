#!/usr/bin/python3
################################################################################
# N queens problem tester
#
# Syntax: python launch.py <step> <max> <timeout> <program>
#
# Executes 'program' with n = { step, 2step, 3step, ..., max } and a max time
# of 'timeout' seconds, writing results to 'output.txt'. 'program' must accept
# an integer and '-test' as arguments and should print two integers to stdout.
#
# Requires Python 3
#
# Copyleft 2013 Vikman - All rights revoked.
# vikman90.blogspot.com - vmfdez90@gmail.com
# February 8, 2013
# 
################################################################################

from sys import argv
from time import clock
from subprocess import check_output, TimeoutExpired

if __name__ == '__main__':
    if len(argv) < 5:
        print('Sintaxis: python launch.py <salto> <max> <tiempo> <programa>')
        exit(1)
        
    step = int(argv[1])
    stop = int(argv[2])
    timemax = float(argv[3])
    program = argv[4:]
    fileout = open('output.txt', 'w')

    nsuccess = 0
    ntotal = 0
    nsteps = 0
    nmillis = 0
    time = clock()

    for n in range(step, stop + 1, step):
        ntotal += 1
        
        try:
            output = check_output(program + ['-test', str(n)], \
                                  timeout = timemax).decode()
            fileout.write(str(n) + '\t' + output)
            print(str(n) + ": " + output[:-1])

            nsuccess += 1
            data = output.split()
            nsteps += int(data[0])
            nmillis += int(data[1])
        except TimeoutExpired:
            fileout.write(str(n) + '\n')
            print(str(n) + ': tiempo agotado.')

    time = clock() - time
    fileout.close()

    print('\n\tResultado:')
    print(str(nsuccess) + '/' + str(ntotal), 'problemas resueltos.')
    print('Total:', nsteps, 'pasos en', nmillis, 'ms.')
    print('Media:', nsteps / nsuccess, 'pasos y', nmillis, 'ms.')
    print('Tiempo de ejecucion:', time, 'seg.')
