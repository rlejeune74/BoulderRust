# BoulderRust
Yet another Boulder Rush, but in Rust

[![Coverage Status](https://coveralls.io/repos/github/rlejeune74/BoulderRust/badge.svg?branch=master)](https://coveralls.io/github/rlejeune74/BoulderRust?branch=master)

[![CI Status](https://github.com/rlejeune74/BoulderRust/workflows/ci/badge.svg)](https://github.com/rlejeune74/BoulderRust/actions/workflows/ci.yml)

[![Generated Doc](https://img.shields.io/badge/Doc-Generated-blue)](https://rlejeune74.github.io/BoulderRust/)



## Game Init

### Initial display

````
[Number of colomn] [Number of row] [Number of Diamands]
For each row
    1 digit per colomn describing the cell by:
    - 'x' => a not movable rock
    - 'o' => a boulder
    - '1..9' => number of diamands
    - '0' => a boulder with diamands (next digit provide the number of diamands)
````

### Add Player

Performed by running the command:
`PLAYER [NAME]`

### Game Start

Performed by running the command:
`GAME START`

## Turn Sequence

### Turn init
````
[Number of player]
For each player
    [Name] [X_POS] [Y_POS] [Number of diamands]
[Board Description as for board init]
````

### Action Order

- `MOVE [UP/DOWN/LEFT/RIGHT]`
- `SHOOT [UP/DOWN/LEFT/RIGHT]`
- `DIG`

### Process Action

Actions are process per priority order MOVE > SHOOT > DIG.
This make it:
- easier to dodge bullet
- harder to dig

### Compute new state

Board is updated according to players action and their new state in the board.