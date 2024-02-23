/*******************************************************************************
 * N queens problem (JavaScript version)
 * Main file
 *
 * Copyleft 2016 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * March 26, 2016
 *
 * Syntax: nodejs queens.js [-test] [N]
 *
 ******************************************************************************/

const libchess = require("./chess")
var n = 0
testing = false

for (i in process.argv)
    if (process.argv[i] === "-test")
        testing = true;
    else
        n = parseInt(process.argv[i])

if (isNaN(n) || n < 4) {
    console.error("Usage: " + process.argv[0] + " [-test] N")
    process.exit(1);
}

var chess = new libchess.Chess(n)
var hrtime = process.hrtime()
var time = hrtime[0] *1e6 + hrtime[1] / 1e3

chess.solve()

hrtime = process.hrtime()
time = parseInt((hrtime[0] *1e6 + hrtime[1] / 1e3 - time))

if (testing)
    console.log(String(chess.steps()) + "\t" + chess.discards() + "\t" + time)
else {
    console.log(String(chess))
    console.log("Solved in " + chess.steps() + " steps. Time: " + time / 1000 + " ms.")
}
