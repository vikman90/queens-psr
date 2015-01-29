/**
 * *****************************************************************************
 * N queens problem (Java version) Main file
 *
 * Copyleft 2013 Vikman - All rights revoked. vikman90.blogspot.com -
 * vmfdez90@gmail.com February 8, 2013
 *
 * Syntax: java queens/Main [-test] [N]
 *
 *****************************************************************************
 */
package queens;

import java.io.BufferedReader;
import java.io.InputStreamReader;

public class Main {

    public static void main(String[] args) {
        Chess chess;
        int n = 0;
        long time;
        boolean testing = false;

        for (String arg : args) {
            if (arg.equals("-test")) {
                testing = true;
            } else {
                n = Integer.parseInt(arg);
            }
        }

        while (n < 4) {
            BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
            System.out.print("Introduzca cantidad de reinas: ");

            try {
                n = Integer.parseInt(reader.readLine());
            } catch (Exception error) {
                continue;
            }
        }

        chess = new Chess(n);
        time = System.currentTimeMillis();
        chess.solve();
        time = System.currentTimeMillis() - time;

        if (testing) {
            System.out.println(chess.getSteps() + "\t" + time);
        } else {
            System.out.println(chess);
            System.out.print("Resuelto en " + chess.getSteps() + " pasos. ");
            System.out.println("Tiempo: " + time + " ms.");
        }
    }
}
