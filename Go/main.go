// Vikman Fernandez-Castro
// April 11, 2020

package main

import (
	"log"
	"math/rand"
	"os"
	"strconv"
	"time"

	"queens/chess"
)

func main() {
	rand.Seed(time.Now().UnixNano())
	size := 4

	if len(os.Args) > 1 {
		s, err := strconv.Atoi(os.Args[1])

		if err != nil {
			log.Fatal(err)
		} else if size < 4 {
			log.Fatal("The chess size must be greater or equal than 4.")
		}

		size = s
	}

	chess := chess.New(size)

	start := time.Now()
	chess.Solve()
	end := time.Now()

	chess.Print()
	log.Println("Steps:      ", chess.GetSteps())
	log.Println("Discards:   ", chess.GetDiscards())
	log.Println("Time:       ", end.Sub(start))
	log.Printf("Performance: %d steps/ms\n", int64(chess.GetSteps()*1000000)/end.Sub(start).Nanoseconds())
	log.Printf("             %d discards/ms\n", int64(chess.GetDiscards()*1000000)/end.Sub(start).Nanoseconds())
}
