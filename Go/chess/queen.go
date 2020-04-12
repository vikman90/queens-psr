// Vikman Fernandez-Castro
// April 11, 2020

package chess

import "math/rand"

type queen struct {
	positions []bool
	count     int
}

func newQueen(size int) *queen {
	queen := new(queen)
	queen.positions = make([]bool, size)

	for i := range queen.positions {
		queen.positions[i] = true
	}

	queen.count = size
	return queen
}

func (queen *queen) getValues() []int {
	values := make([]int, 0, len(queen.positions))
	offset := rand.Int()

	for i := range queen.positions {
		index := (i + offset) % len(queen.positions)

		if queen.positions[index] {
			values = append(values, index)
		}
	}

	return values
}

func (queen *queen) setZero() {
	queen.positions = make([]bool, len(queen.positions))
	queen.count = 0
}

func (queen *queen) setValue(value int) {
	queen.positions[value] = true
	queen.count++
}

func (queen *queen) setValues(values []int) {
	queen.positions = make([]bool, len(queen.positions))

	for _, v := range values {
		queen.positions[v] = true
	}

	queen.count = len(values)
}

func (queen *queen) unsetValue(value int) {
	if !queen.positions[value] {
		panic("Cannot unset a value from a queen.")
	}

	queen.positions[value] = false
	queen.count--
}
