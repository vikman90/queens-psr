<?php
/*******************************************************************************
 * N queens problem (PHP version)
 * Main file
 *
 * Copyleft 2016 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * March 24, 2016
 *
 * Syntax: php queens.php [-test] [N]
 *
 ******************************************************************************/

require "chess.php";

$n = 0;
$testing = false;

if (!isset($argv)) {
    echo "Error. Run this program from CLI.\n";
    exit;
}

foreach ($argv as $a) {
    if ($a === '-test')
        $testing = true;
    else
        $n = (int)$a;
}

while ($n < 4)
    $n = (int)readline("Indique cantidad de reinas: ");

$chess = new Chess($n);
$time = microtime(true);
$chess->solve();
$time = (int)((microtime(true) - $time) * 1000);
$steps = $chess->steps();

if ($testing)
    echo "$steps\t$time\n";
else {
    echo $chess;
    echo "Resuelto en $steps pasos. Tiempo: $time ms.\n";
}
