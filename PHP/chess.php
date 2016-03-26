<?php
/*******************************************************************************
 * N queens problem (PHP version)
 * Class Chess
 *
 * Copyleft 2016 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * March 24, 2016
 *
 ******************************************************************************/

mt_srand();

class Chess {
    private $size;
    private $queens;
    private $nsteps;
    private $stackDiscarded;
    private $stackCount;

    //--------------------------------------------------------------------------
    // Constructor

    public function __construct($size) {
        $this->size = $size;
        $this->queens = array_fill(0, $size, array_fill(0, $size, 1));
        $this->nsteps = 0;
        $this->stackDiscarded = [];
        $this->stackCount = [];
    }

    //--------------------------------------------------------------------------
    // Solve queens problem

    public function solve() {
        $index = $this->selectIndex();

        if ($index == -1)
            return true;

        $row = $this->queens[$index];
        $values = $this->selectValues($row);

        foreach ($values as $value) {
            if (!$this->assign($index, $value)) {
                $this->queens[$index] = $row;
                continue;
            }

            if ($this->solve())
                return true;

            $this->restoreLast();
            $this->queens[$index] = $row;
        }

        return false;
    }

    //--------------------------------------------------------------------------
    // Get index of a unsolved row (minimum remaining values heuristics)

    private function selectIndex() {
        $minSize = $this->size + 1;
        $index = -1;

        for ($i = 0; $i < $this->size; $i++) {
            $curSize = count($this->queens[$i]);

            if ($curSize > 1 and $curSize < $minSize) {
                $index = $i;
                $minSize = $curSize;
            }
        }

        return $index;
    }

    //--------------------------------------------------------------------------
    // Assign a value to a row and propagate constraints

    private function assign($index, $value) {
        $this->queens[$index] = [];
        $this->stackCount[] = 0;
        $this->nsteps++;

        for ($i = 0; $i < $this->size; $i++) {
            if ($i == $index)
                continue;

            $diag1 = $value + ($index - $i);
            $diag2 = $value - ($index - $i);

            if (!($this->discard($i, $value) and $this->discard($i, $diag1) and
                  $this->discard($i, $diag2))) {
                $this->restoreLast();
                return false;
            }
        }

        $this->queens[$index][$value] = 1;
        return true;
    }

    //--------------------------------------------------------------------------
    // Discard candidate values (constraints propagation)

    private function discard($index, $value) {
        $row = $this->queens[$index];

        if (!isset($row[$value]))
            return true;

        unset($row[$value]);
        $this->stackDiscarded[] = [$index, $value];
        $this->stackCount[count($this->stackCount) - 1]++;

        if (count($row) == 0)
            return false;

        if (count($row) == 1) {
            $value = array_keys($row)[0];

            for ($i = 0; $i < $this->size; $i++) {
                if ($i == $index)
                    continue;

                $diag1 = $value + ($index - $i);
                $diag2 = $value - ($index - $i);

                if (!($this->discard($i, $value) and $this->discard($i, $diag1) and
                      $this->discard($i, $diag2))) {
                    $this->restoreLast();
                    return false;
                }
            }
        }

        return true;
    }

    //--------------------------------------------------------------------------
    // Undo last assignation (restore constraints)

    private function restoreLast() {
        $n = array_pop($this->stackCount);

        for ($i = 0; $i < $n; $i++) {
            $pair = array_pop($this->stackDiscarded);
            $this->queens[$pair[0]][$pair[1]] = 1;
        }
    }

    //--------------------------------------------------------------------------
    // Get number of total tries of assignation

    private function selectValues($array) {
        $values = [];
        $offset = mt_rand(0, $this->size - 1);

        for ($i = 0; $i < count($array); $i++) {
            $index = ($offset + $i) % $this->size;

            if (isset($array[$index]))
                $values[] = $index;
        }

        return $values;
    }

    //--------------------------------------------------------------------------
    // Get number of total tries of assignation

    public function steps() {
        return $this->nsteps;
    }

    //--------------------------------------------------------------------------
    // Representation of the chessboard

    public function __toString() {
        $string = "";

        for ($i = 0; $i < $this->size; $i++) {
            $j = $i + 1;
            $value = array_keys($this->queens[$i])[0] + 1;
            $string .= "Reina $j: $value\n";
        }

        return $string;
    }
}
