# N Queens Problem Solver

[![CI](https://github.com/vikman90/queens-psr/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/vikman90/queens-psr/actions/workflows/ci.yml)

This is a set of implementations of the famous N Queens Problem solver in
multiple programming languages. The N Queens Problem is a problem of placing N
queens on an NxN chessboard without any queen being able to attack the others.

## Features

- Efficient implementations.
- Utilize backtracking and constraint propagation to find a solution.
- Provides the ability to specify the number of queens (size of the board).
- Offers an alternative testing output of the found solutions.

## Performance comparison

|Implementation|Steps/ms (avg)|Discards/ms (avg)|
|--|--:|--:|
|C++|125,20|10.715,92|
|Rust|93,10|5.793,85|
|Go|72,91|1.282,14|
|JavaScript|29,49|1.629,27|
|C#|22,65|1.263,75|
|Java|13,26|1.047,12|
|Python|2,17|519,03|

## Usage

Instructions available at each implementation folder.

## Contribution

Contributions are welcome! If you have any ideas to improve this project, feel free to submit a pull request.

## Credits

This project was developed by [Vikman Fernandez-Castro](https://github.com/vikman90).

## License

This project is licensed under the MIT License.
