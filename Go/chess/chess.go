// Vikman Fernandez-Castro
// April 11, 2020

package chess

import "fmt"

type chess struct {
	nSteps    int
	nDiscards int
	queens    []*queen
	discarded [][2]int
}

func New(size int) *chess {
	chess := new(chess)
	chess.queens = make([]*queen, size)

	for i := range chess.queens {
		chess.queens[i] = newQueen(size)
	}

	chess.discarded = make([][2]int, 0)
	return chess
}

func (chess *chess) Solve() bool {
	index := chess.selectIndex()

	if index < 0 {
		return true
	}

	candidates := chess.queens[index].getValues()

	for _, value := range candidates {
		if !chess.assign(index, value) {
			chess.queens[index].setValues(candidates)
			continue
		}

		if chess.Solve() {
			return true
		}

		chess.restore()
		chess.queens[index].setValues(candidates)
	}

	return false
}

func (chess *chess) assign(index, value int) bool {
	chess.nSteps++
	chess.discarded = append(chess.discarded, [2]int{-1, -1})

	queen := chess.queens[index]
	queen.setZero()

	if !chess.restrict(index, value) {
		chess.restore()
		return false
	}

	queen.setValue(value)
	return true
}

func (chess *chess) restrict(index, value int) bool {
	for i := range chess.queens {
		if i == index {
			continue
		}

		values := []int{value, value + (index - i), value - (index - i)}

		for _, v := range values {
			if !chess.discard(i, v) {
				return false
			}
		}
	}

	return true
}

func (chess *chess) discard(index, value int) bool {
	if value < 0 || value >= len(chess.queens) || chess.queens[index].positions[value] == false {
		return true
	}

	chess.nDiscards++
	chess.discarded = append(chess.discarded, [2]int{index, value})

	queen := chess.queens[index]
	queen.unsetValue(value)

	switch queen.count {
	case 0:
		return false

	case 1:
		value := queen.findValue()

		if !chess.restrict(index, value) {
			return false
		}
	}

	return true
}

func (chess *chess) restore() {
	for {
		pair := chess.discarded[len(chess.discarded)-1]
		chess.discarded = chess.discarded[:len(chess.discarded)-1]

		if pair[0] == -1 {
			return
		}

		chess.queens[pair[0]].setValue(pair[1])
	}
}

func (chess *chess) selectIndex() int {
	min := len(chess.queens) + 1
	index := -1

	for i, q := range chess.queens {
		if q.count > 1 && q.count < min {
			index = i
			min = q.count
		}
	}

	return index
}

func (chess *chess) Print() {
	for i, q := range chess.queens {
		switch q.count {
		case 0:
			fmt.Printf("[%d] (invalid)\n", i)

		case 1:
			fmt.Printf("[%d] %d\n", i, q.findValue())

		default:
			fmt.Printf("[%d] (unsolved)\n", i)
		}
	}
}

func (chess *chess) GetSteps() int {
	return chess.nSteps
}

func (chess *chess) GetDiscards() int {
	return chess.nDiscards
}
