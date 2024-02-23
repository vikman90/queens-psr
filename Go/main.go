// Vikman Fernandez-Castro
// April 11, 2020

package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"time"

	"queens/chess"
)

func main() {
	size := 4
	testing := false

	for _, arg := range os.Args[1:] {
		if arg == "-test" {
			testing = true
		} else {
			s, err := strconv.Atoi(arg)

			if err != nil {
				log.Fatal(err)
			} else if size < 4 {
				log.Fatal("The chess size must be greater or equal than 4.")
			}

			size = s
		}
	}

	chess := chess.New(size)

	start := time.Now()
	chess.Solve()
	end := time.Now()

	if testing {
		fmt.Printf("%d\t%d\t%d\n", chess.GetSteps(), chess.GetDiscards(), end.Sub(start).Microseconds())
	} else {
		chess.Print()
		fmt.Println("Steps:      ", chess.GetSteps())
		fmt.Println("Discards:   ", chess.GetDiscards())
		fmt.Println("Time:       ", end.Sub(start))
		fmt.Printf("Performance: %d steps/ms\n", int64(chess.GetSteps()*1000000)/end.Sub(start).Nanoseconds())
		fmt.Printf("             %d discards/ms\n", int64(chess.GetDiscards()*1000000)/end.Sub(start).Nanoseconds())
	}
}
