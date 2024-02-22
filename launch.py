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

from sys import argv, exit, stdout
from time import perf_counter
from subprocess import check_output, TimeoutExpired, CalledProcessError

if __name__ == '__main__':
    if len(argv) < 5:
        print('Usage:', argv[0], '<step> <stop> <secs> <tries> <program>')
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
    time = perf_counter()

    for n in range(step, stop + 1, step):
        try:
            for _ in range(ntries):
                nsteps = 0
                ndiscards = 0
                nmillis = 0

                try:
                    print(f'{n}: ', end='')
                    stdout.flush()
                    output = check_output(program + ['-test', str(n)], \
                                          timeout = timemax).decode()
                    fileout.write(str(n) + '\t' + output)
                    print(output[:-1])

                    nsuccess += 1
                    data = output.split()
                    nsteps = int(data[0])
                    ndiscards = int(data[1])
                    nmillis = int(data[2]) / 1000
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
                    print('[Timeout]')
                    continue

                except CalledProcessError:
                    print('[Program error]')
                    continue

            if not nsteps:
                fileout.write(str(n) + '\n')

        except KeyboardInterrupt:
            break

    time = perf_counter() - time
    fileout.close()

    print('\n================================ Results ================================\n')
    print(f'{nsuccess} / {ntotal} programs solved')

    if nsuccess == 0 or summillis == 0:
        exit(1)

    print(f'Total: {sumsteps} steps, {sumdiscards} discards, {summillis:.03f} ms.')
    print(f'Mean: {(sumsteps / nsuccess):.0f} steps, {(sumdiscards / nsuccess):.0f} discards, {(summillis / nsuccess):.0f} ms.')
    print(f'Maximum: {maxsteps} steps, {maxdiscards} discards, {maxmillis} ms.')
    print(f'Performance: {(sumsteps / summillis):.0f} steps/ms, {(sumdiscards / summillis):.0f} discards/ms.')
    print(f'Runtime (launcher): {time:0.3f} sec.')
