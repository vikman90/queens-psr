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
from subprocess import check_output, TimeoutExpired, CalledProcessError

if __name__ == '__main__':
    if len(argv) < 5:
        print('Uso', argv[0], '<salto> <max> <tiempo> <intentos> <programa>')
        exit(1)

    step = int(argv[1])
    stop = int(argv[2])
    timemax = float(argv[3])
    ntries = int(argv[4])
    program = argv[5:]
    fileout = open('output.txt', 'w')

    ntotal = stop // step
    nsuccess = 0
    sumsteps = 0
    sumdiscards = 0
    summillis = 0
    maxsteps = 0
    maxdiscards = 0
    maxmillis = 0
    time = clock()

    for n in range(step, stop + 1, step):
        try:
            for _ in range(ntries):
                nsteps = 0
                ndiscards = 0
                nmillis = 0

                try:
                    output = check_output(program + ['-test', str(n)], \
                                          timeout = timemax).decode()
                    fileout.write(str(n) + '\t' + output)
                    print(str(n) + ": " + output[:-1])

                    nsuccess += 1
                    data = output.split()
                    nsteps = int(data[0])
                    ndiscards = int(data[1])
                    nmillis = int(data[2])
                    sumsteps += nsteps
                    sumdiscards += ndiscards
                    summillis += nmillis

                    if nsteps > maxsteps:
                        maxsteps = nsteps

                    if ndiscards > maxdiscards:
                        maxdiscards = ndiscards

                    if nmillis > maxmillis:
                        maxmillis = nmillis

                    break

                except TimeoutExpired:
                    print(str(n) + ': tiempo agotado.')
                    continue

                except CalledProcessError:
                    print(str(n) + ': error del programa.')
                    continue

            if not nsteps:
                fileout.write(str(n) + '\n')

        except KeyboardInterrupt:
            break

    time = clock() - time
    fileout.close()

    print('\n\tResultado:')
    print('{} / {} problemas resueltos'.format(nsuccess, ntotal))
    print('Total: {} pasos, {} descartes, {} ms.'.format( \
        nsteps, ndiscards, nmillis))
    print('Media: {} pasos, {} descartes, {} ms.'.format( \
        round(sumsteps / nsuccess), round(sumdiscards / nsuccess), \
        round(summillis / nsuccess)))
    print("Maximo: {} pasos, {} descartes, {} ms.".format( \
        maxsteps, maxdiscards, maxmillis))
    print('Eficiencia: {} pasos/ms, {} descartes/ms.'.format( \
        round(maxsteps / maxmillis), round(maxdiscards / maxmillis)))
    print('Tiempo de ejecucion (launcher): {} seg.'.format(time))
