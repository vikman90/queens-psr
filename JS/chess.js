/*******************************************************************************
 * N queens problem (JavaScript version)
 * Class Chess
 *
 * Copyleft 2016 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * March 26, 2016
 *
 ******************************************************************************/

function fill(length, value) {
    var array = new Array(value)

    for (var i = 0; i < length; i++)
        array[i] = value

    return array
}

function Chess(size) {

    //--------------------------------------------------------------------------
    // Constructor

    var complete = fill(size, 1)

    this.size = size
    this.queens = new Array(size)
    this.queensCount = fill(size, size)
    this.nsteps = 0
    this.ndiscards = 0
    this.stackDiscarded = []
    this.stackCount = []

    for (var i = 0; i < size; i++)
        this.queens[i] = complete.slice()

    //--------------------------------------------------------------------------
    // Solve queens problem

    this.solve = function() {
        var index = this.selectIndex()

        if (index == -1)
            return true

        var row = this.queens[index].slice()
        var values = this.selectValues(row)

        for (var i in values) {
            var value = values[i]

            if (!this.assign(index, value)) {
                this.queens[index] = row.slice()
                this.queensCount[index] = values.length
                continue
            }

            if (this.solve())
                return true

            this.restoreLast()
            this.queens[index] = row.slice()
            this.queensCount[index] = values.length
        }

        return false
    }

    //--------------------------------------------------------------------------
    // Get index of a unsolved row (minimum remaining values heuristics)

    this.selectIndex = function() {
        var minsize = this.size + 1
        var index = -1

        for (var i = 0; i < this.size; i++) {
            var cursize = this.queensCount[i]

            if (cursize > 1 && cursize < minsize) {
                index = i
                minsize = cursize
            }
        }

        return index
    }

    //--------------------------------------------------------------------------
    // Assign a value to a row and propagate constraints

    this.assign = function(index, value) {
        this.queens[index] = fill(this.size, 0)
        this.queensCount[index] = 0
        this.stackCount.push(0)
        this.nsteps++

        for (var i = 0; i < this.size; i++) {
            if (i == index)
                continue

            var diag1 = value + (index - i)
            var diag2 = value - (index - i)

            if (!(this.discard(i, value) && this.discard(i, diag1) && this.discard(i, diag2))) {
                this.restoreLast()
                return false
            }
        }

        this.queens[index][value] = 1
        this.queensCount[index] = 1
        return true
    }

    //--------------------------------------------------------------------------
    // Discard candidate values (constraints propagation)

    this.discard = function(index, value) {
        if (value < 0 || value >= this.size || !this.queens[index][value])
            return true

        this.ndiscards++
        this.queens[index][value] = 0
        this.queensCount[index]--
        this.stackDiscarded.push([index, value])
        this.stackCount[this.stackCount.length - 1]++

        if (this.queensCount[index] == 0)
            return false

        if (this.queensCount[index] == 1) {
            value = this.queens[index].indexOf(1)

            for (var i = 0; i < this.size; i++) {
                if (i == index)
                    continue

                var diag1 = value + (index - i)
                var diag2 = value - (index - i)

                if (!(this.discard(i, value) && this.discard(i, diag1) && this.discard(i, diag2)))
                    return false
            }
        }

        return true
    }

    //--------------------------------------------------------------------------
    // Undo last assignation (restore constraints)

    this.restoreLast = function() {
        var n = this.stackCount.pop()

        for (var i = 0; i < n; i++) {
            var pair = this.stackDiscarded.pop()

            if (!this.queens[pair[0]][pair[1]]) {
                this.queens[pair[0]][pair[1]] = 1
                this.queensCount[pair[0]]++
            }
        }
    }

    //--------------------------------------------------------------------------
    // Select all available indices from a row

    this.selectValues = function(row) {
        var values = []
        var offset = parseInt(Math.random() * this.size)

        for (var i = 0; i < row.length; i++) {
            var index = (offset + i) % this.size

            if (row[index])
                values.push(index)
        }

        return values
    }

    //--------------------------------------------------------------------------
    // Representation of the chessboard

    this.toString = function() {
        var string = ""

        for (var i = 0; i < this.size; i++) {
            if (this.queensCount[i] != 1)
                string += "Queen " + (i + 1) + " not solved\n"
            else
                string += "Queen " + (i + 1) + ": square " + (this.queens[i].indexOf(1) + 1) + "\n"
        }

        return string
    }

    //--------------------------------------------------------------------------
    // Get number of total tries of assignation

    this.steps = function() {
        return this.nsteps
    }
    
    //--------------------------------------------------------------------------
    // Get number of total discards

    this.discards = function() {
        return this.ndiscards
    }
}

module.exports.Chess = Chess
