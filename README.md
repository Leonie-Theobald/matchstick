# Matchstick
A 100% Rust project for solving and generating matchstick puzzles.
Code examples can be found in [main.rs](src/main.rs)

## Solving Matchstick Puzzle
The user states the riddle equation composed of matchsticks and the and
the required number of movements of matchsticks.

The program follows this procedure:
- generate all possible transitions of the matchsticks
- filter transitions for syntactically valid equation - meaning the
matchsticks form known "symbols", like digits or operators (+ - =)
- filter syntactically valid equations for mathematically true
equations - both sides of the equation have the same result
- return collection of all valid and true equations

## Generating Matchstick Puzzles
The user states the equation pattern for the riddle, the required
number of movements of matchsticks, the number of solutions the
puzzle should have, and optionally the equation pattern that the
solution equation must fulfill.

An equation pattern is a sequence of filter options that describe
what symbols are allowed at this position.
E.g. only numbers or only operators.

The program follows this procedure:
- generate all concrete equations matching the riddle equation pattern
- go through equations, solve them, and filter for correct solution
count
- if the solution equation pattern is given, filter for riddles where
all solutions match this pattern
- return collection of puzzles
