use crate::equation::{Equation, EquationPattern};

/// Holds information to describe a matchstick riddle
#[derive(Debug, PartialEq)]
pub struct Riddle {
    riddle_equation: Equation,
    number_matchstick_movements: usize,
}

impl Riddle {
    /// Creates new [`Riddle`]
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::puzzle::Riddle;
    /// # use matchstick::symbol::Symbol;
    /// let equation = Equation::new_from_symbols(vec![
    ///     Symbol::Two,
    ///     Symbol::Plus,
    ///     Symbol::Five,
    ///     Symbol::Equal,
    ///     Symbol::Nine,
    /// ]);
    /// // Creates a [`Riddle`] that asks to solve the given [`Equation`] with 2 matchstick movements
    /// let riddle = Riddle::new(equation, 2);
    /// ```
    pub fn new(equation: Equation, number_matchstick_movements: usize) -> Self {
        Riddle {
            riddle_equation: equation,
            number_matchstick_movements,
        }
    }

    // Programmatically search for a solution
    fn solve(&self) -> SolutionWrapper {
        let transformed_equations = self
            .riddle_equation
            .move_n_matchsticks(self.number_matchstick_movements);
        let solution_equations = transformed_equations
            .into_iter()
            .filter_map(|equation| match equation.mathematically_validate() {
                Ok(()) => Some(equation),
                Err(()) => None,
            })
            .collect();

        SolutionWrapper::new_programmatically_set_solution(solution_equations)
    }
}

/// Holds information to describe the solution of a matchstick riddle
#[derive(Debug, PartialEq)]
pub struct Solution {
    solution_equations: Vec<Equation>,
}

/// Wraps [`Solution`]s of a [`Riddle`]
#[derive(Debug, PartialEq)]
pub enum SolutionWrapper {
    NotYetSet,
    ProgrammaticallySet(Solution),
    ManuallySet(Solution),
}

impl SolutionWrapper {
    /// Creates new [`Solution`] encapsulated in [`SolutionWrapper`]
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::puzzle::Riddle;
    /// # use matchstick::puzzle::Solution;
    /// # use matchstick::puzzle::SolutionWrapper;
    /// # use matchstick::symbol::Symbol;
    /// let equation = Equation::new_from_symbols(vec![
    ///     Symbol::Two,
    ///     Symbol::Plus,
    ///     Symbol::Five,
    ///     Symbol::Equal,
    ///     Symbol::Nine,
    /// ]);
    ///
    /// let wrapped_solution = SolutionWrapper::new_manually_set_solution(vec![equation]);
    /// assert!(matches!(
    ///     wrapped_solution,
    ///     SolutionWrapper::ManuallySet(Solution { .. })
    /// ));
    /// ```
    pub fn new_manually_set_solution(solution_equations: Vec<Equation>) -> Self {
        SolutionWrapper::ManuallySet(Solution { solution_equations })
    }

    fn new_programmatically_set_solution(solution_equations: Vec<Equation>) -> Self {
        SolutionWrapper::ProgrammaticallySet(Solution { solution_equations })
    }

    // unwrap the wrapper
    // This can panic. Only use this if solution is already set
    fn get_inner_reference(&self) -> Result<&Solution, ()> {
        match self {
            SolutionWrapper::NotYetSet => Err(()),
            SolutionWrapper::ProgrammaticallySet(solution)
            | SolutionWrapper::ManuallySet(solution) => Ok(solution),
        }
    }
}

/// Holds the [`Riddle`] and the [`SolutionWrapper`] containing the [`Solution`]
#[derive(Debug, PartialEq)]
pub struct Puzzle {
    riddle: Riddle,
    wrappped_solution: SolutionWrapper,
}

impl Puzzle {
    /// Creates new [`Puzzle`] from riddle while solution is unknown yet.
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::puzzle::Puzzle;
    /// # use matchstick::puzzle::Riddle;
    /// # use matchstick::puzzle::SolutionWrapper;
    /// # use matchstick::symbol::Symbol;
    /// // Riddle equation is "2 + 5 = 9" and two matchsticks should be moved.
    /// let riddle = Riddle::new(
    ///     Equation::new_from_symbols(vec![
    ///         Symbol::Two,
    ///         Symbol::Plus,
    ///         Symbol::Five,
    ///         Symbol::Equal,
    ///         Symbol::Nine,
    ///     ]),
    ///     2,
    /// );
    /// let puzzle = Puzzle::new_from_riddle(riddle);
    ///
    /// assert_eq!(puzzle.get_wrapped_solution(), &SolutionWrapper::NotYetSet);
    /// ```
    pub fn new_from_riddle(riddle: Riddle) -> Self {
        Puzzle {
            riddle,
            wrappped_solution: SolutionWrapper::NotYetSet,
        }
    }

    /// Programmatically find solution in form of [`Equation`]s fitting to the [`Riddle`] of this [`Puzzle`]\
    /// The found solution is set\
    /// Returns number of found solution [`Equation`]s
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::puzzle::Puzzle;
    /// # use matchstick::puzzle::Riddle;
    /// # use matchstick::puzzle::SolutionWrapper;
    /// # use matchstick::symbol::Symbol;
    /// // Equation is "7 - 3 = 4" and one matchstick must be moved
    /// let riddle = Riddle::new(
    ///     Equation::new_from_symbols(vec![
    ///         Symbol::Seven,
    ///         Symbol::Minus,
    ///         Symbol::Three,
    ///         Symbol::Equal,
    ///         Symbol::FourVar1,
    ///     ]),
    ///     1,
    /// );
    /// let mut puzzle = Puzzle::new_from_riddle(riddle);
    ///
    /// // There is the only possibility to form a new, mathematically valid equation
    /// // One matchstick moves from seven to minus changing to one and plus
    /// assert_eq!(1, puzzle.search_and_set_solution());
    /// let solution_equations = vec![Equation::new_from_symbols(vec![
    ///     Symbol::OneVar1,
    ///     Symbol::Plus,
    ///     Symbol::Three,
    ///     Symbol::Equal,
    ///     Symbol::FourVar1,
    /// ])];
    /// assert!(matches!(
    ///     puzzle.get_wrapped_solution(),
    ///     SolutionWrapper::ProgrammaticallySet(solution_equations)
    /// ));
    /// ```
    pub fn search_and_set_solution(&mut self) -> usize {
        self.wrappped_solution = self.riddle.solve();
        match &self.wrappped_solution {
            SolutionWrapper::NotYetSet => panic!("fn solve() should have just set the solutions"),
            SolutionWrapper::ProgrammaticallySet(solution)
            | SolutionWrapper::ManuallySet(solution) => solution.solution_equations.len(),
        }
    }

    /// The [`Solution`] of the [`Puzzle`] is set to given, arbitrary value
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::puzzle::Puzzle;
    /// # use matchstick::puzzle::Riddle;
    /// # use matchstick::puzzle::Solution;
    /// # use matchstick::puzzle::SolutionWrapper;
    /// # use matchstick::symbol::Symbol;
    /// # let riddle = Riddle::new(
    ///     Equation::new_from_symbols(vec![
    ///         Symbol::Two,
    ///         Symbol::Plus,
    ///         Symbol::Five,
    ///         Symbol::Equal,
    ///         Symbol::Nine,
    ///     ]),
    ///     2,
    /// );
    /// let mut puzzle = Puzzle::new_from_riddle(riddle);
    /// let wrapped_solution = SolutionWrapper::new_manually_set_solution(Vec::new());
    /// puzzle.manually_set_solution(wrapped_solution);
    /// assert!(matches!(
    ///     puzzle.get_wrapped_solution(),
    ///     &SolutionWrapper::ManuallySet(Solution{..})
    /// ));
    ///
    /// let wrapped_solution = SolutionWrapper::new_manually_set_solution(vec![
    ///     Equation::new_from_symbols(vec![Symbol::Three]),
    /// ]);
    /// puzzle.manually_set_solution(wrapped_solution);
    /// assert!(matches!(
    ///     puzzle.get_wrapped_solution(),
    ///     &SolutionWrapper::ManuallySet(Solution{..})
    /// ));
    /// ```
    pub fn manually_set_solution(&mut self, wrapped_solution: SolutionWrapper) {
        self.wrappped_solution = wrapped_solution;
    }

    /// Getter function for [`Riddle`]  
    pub fn get_riddle(&self) -> &Riddle {
        &self.riddle
    }

    /// Getter function for solution wrapped in [`SolutionWrapper`]  
    pub fn get_wrapped_solution(&self) -> &SolutionWrapper {
        &self.wrappped_solution
    }

    /// Setter function for [`Riddle`]  
    pub fn set_riddle(&mut self, riddle: Riddle) {
        self.riddle = riddle
    }
}

/// Searches for specific [`Puzzle`]s where [`Riddle`] (and solution) fulfill given general [`EquationPattern`]
pub struct PuzzleGenerator {
    riddle_equation_pattern: EquationPattern,
    number_matchstick_movements: usize,
    solution_equation_pattern: Option<EquationPattern>,
}

impl PuzzleGenerator {
    /// The [`EquationPattern`] describes riddle equation and the [`usize`] tells
    /// how many matchsticks must be moved to solve the [`Puzzle`] generated by [`PuzzleGenerator`]
    /// ```
    /// # use matchstick::equation::EquationPattern;
    /// # use matchstick::puzzle::PuzzleGenerator;
    /// # use matchstick::symbol::SymbolFilter;
    /// let equation_pattern = EquationPattern::new_from_symbol_filters(vec![SymbolFilter::IsNumber]);
    /// let puzzle_generator = PuzzleGenerator::new(equation_pattern, 2);
    /// // solution pattern is set to `None` by default
    /// assert_eq!(&None, puzzle_generator.get_solution_equation_pattern());
    /// ```
    pub fn new(
        riddle_equation_pattern: EquationPattern,
        number_matchstick_movements: usize,
    ) -> Self {
        PuzzleGenerator {
            riddle_equation_pattern,
            number_matchstick_movements,
            solution_equation_pattern: None,
        }
    }

    /// Find all [`Puzzle`]s where the riddle matches the given pattern and has only n solutions \
    /// If the solution pattern is given, this only returns [`Puzzle`]s where the solution matches
    pub fn derive_puzzles_with_n_solutions(&self, number_solutions: usize) -> Vec<Puzzle> {
        let mut puzzles = Vec::new();

        'outer: for riddle_equation in self.riddle_equation_pattern.derive_concrete_equations() {
            // for each starting equation a new puzzle is set up to be solved then
            let mut puzzle = Puzzle::new_from_riddle(Riddle::new(
                riddle_equation,
                self.number_matchstick_movements,
            ));

            if number_solutions != puzzle.search_and_set_solution() {
                continue; // the riddle_equation has not requested number of solutions
            }

            // if solution pattern is set, all solution equations must fulfill it
            // in order for the riddle equation to be valid
            if let Some(solution_equation_pattern) = &self.solution_equation_pattern {
                let solution = match puzzle.wrappped_solution.get_inner_reference() {
                    Ok(solution_equations) => solution_equations,
                    Err(()) => continue 'outer, // go to next riddle equation,
                };
                for solution_equation in &solution.solution_equations {
                    if !solution_equation.fulfills_abstract_equation(solution_equation_pattern) {
                        continue 'outer; // go to next riddle equation
                    }
                }
            }

            puzzles.push(puzzle);
        }

        puzzles
    }

    /// Setter function for number of matchstick movements
    pub fn set_riddle_equation_pattern(&mut self, riddle_equation_pattern: EquationPattern) {
        self.riddle_equation_pattern = riddle_equation_pattern;
    }

    /// Setter function for number of matchstick movements
    pub fn set_number_matchstick_movements(&mut self, number_matchstick_movements: usize) {
        self.number_matchstick_movements = number_matchstick_movements;
    }

    /// Set what [`EquationPattern`] the solution equations must fulfill. Only then the specific [`Riddle`] and
    /// solution form a valid [`Puzzle`] generated by [`PuzzleGenerator`]
    /// ```
    /// # use matchstick::equation::EquationPattern;
    /// # use matchstick::puzzle::PuzzleGenerator;
    /// # use matchstick::symbol::SymbolFilter;
    /// # let riddle_equation_pattern = EquationPattern::new_from_symbol_filters(vec![SymbolFilter::IsNumber]);
    /// let solution_equation_pattern =
    ///     EquationPattern::new_from_symbol_filters(vec![SymbolFilter::IsNumber]);
    /// let mut puzzle_generator = PuzzleGenerator::new(riddle_equation_pattern, 1);
    /// puzzle_generator.set_solution_equation_pattern(solution_equation_pattern);
    /// assert!(matches!(
    ///     puzzle_generator.get_solution_equation_pattern(),
    ///     &Some(EquationPattern { .. })
    /// ));
    /// ```
    pub fn set_solution_equation_pattern(&mut self, solution_equation_pattern: EquationPattern) {
        self.solution_equation_pattern = Some(solution_equation_pattern);
    }

    /// Getter function for riddle [`EquationPattern`]
    pub fn get_riddle_equation_pattern(&self) -> &EquationPattern {
        &self.riddle_equation_pattern
    }

    /// Getter function for number of matchstick movements
    pub fn get_number_matchstick_movements(&self) -> &usize {
        &self.number_matchstick_movements
    }

    /// Getter function for solution [`EquationPattern`]
    pub fn get_solution_equation_pattern(&self) -> &Option<EquationPattern> {
        &self.solution_equation_pattern
    }
}

#[cfg(test)]
mod test {
    use crate::symbol::{Symbol, SymbolFilter};

    use super::*;

    #[test]
    fn set_new_riddle() {
        let riddle_equation = Equation::new_from_symbols(vec![
            Symbol::FourVar2,
            Symbol::Two,
            Symbol::Plus,
            Symbol::Nine,
            Symbol::Equal,
            Symbol::OneVar1,
        ]);

        let mut puzzle = Puzzle {
            riddle: Riddle {
                riddle_equation: riddle_equation.clone(),
                number_matchstick_movements: 2,
            },
            wrappped_solution: SolutionWrapper::NotYetSet,
        };

        let expected_puzzle = Puzzle {
            riddle: Riddle {
                riddle_equation: riddle_equation.clone(),
                number_matchstick_movements: 4,
            },
            wrappped_solution: SolutionWrapper::NotYetSet,
        };

        puzzle.set_riddle(Riddle {
            riddle_equation,
            number_matchstick_movements: 4,
        });
        assert_eq!(expected_puzzle, puzzle);
    }

    #[test]
    fn test_leading_minus() {
        let riddle_equation = Equation::new_from_symbols(vec![
            Symbol::FourVar2,
            Symbol::Two,
            Symbol::Plus,
            Symbol::Nine,
            Symbol::Equal,
            Symbol::OneVar1,
        ]);

        let mut puzzle = Puzzle {
            riddle: Riddle {
                riddle_equation,
                number_matchstick_movements: 2,
            },
            wrappped_solution: SolutionWrapper::NotYetSet,
        };

        let expected_solution =
            SolutionWrapper::new_programmatically_set_solution(vec![Equation::new_from_symbols(
                vec![
                    Symbol::Minus,
                    Symbol::EightVar1,
                    Symbol::Plus,
                    Symbol::Nine,
                    Symbol::Equal,
                    Symbol::OneVar1,
                ],
            )]);

        // Only finds one solution equation
        assert_eq!(1, puzzle.search_and_set_solution());
        assert_eq!(expected_solution, puzzle.wrappped_solution);
    }

    #[test]
    fn generate_puzzle_with_no_matchstick_movement() {
        let riddle_pattern = EquationPattern::new_from_symbol_filters(vec![
            SymbolFilter::IsNumber,
            SymbolFilter::List(vec![Symbol::Equal]),
            SymbolFilter::List(vec![Symbol::Three]),
        ]);

        let puzzle_generator = PuzzleGenerator {
            riddle_equation_pattern: riddle_pattern,
            solution_equation_pattern: None,
            number_matchstick_movements: 0,
        };

        let expected_puzzles = vec![Puzzle {
            riddle: Riddle {
                riddle_equation: Equation::new_from_symbols(vec![
                    Symbol::Three,
                    Symbol::Equal,
                    Symbol::Three,
                ]),
                number_matchstick_movements: 0,
            },
            wrappped_solution: SolutionWrapper::new_programmatically_set_solution(vec![
                Equation::new_from_symbols(vec![Symbol::Three, Symbol::Equal, Symbol::Three]),
            ]),
        }];

        assert_eq!(
            puzzle_generator.derive_puzzles_with_n_solutions(1),
            expected_puzzles
        );
    }

    #[test]
    fn no_matching_puzzle() {
        let riddle_pattern = EquationPattern::new_from_symbol_filters(vec![
            SymbolFilter::List(vec![Symbol::Nine]),
            SymbolFilter::List(vec![Symbol::Equal]),
            SymbolFilter::List(vec![Symbol::Three]),
        ]);

        let puzzle_generator = PuzzleGenerator {
            riddle_equation_pattern: riddle_pattern,
            solution_equation_pattern: None,
            number_matchstick_movements: 1,
        };

        assert_eq!(
            puzzle_generator.derive_puzzles_with_n_solutions(1),
            Vec::new()
        );
    }

    #[test]
    fn change_number_matchstick_movements() {
        let equation_pattern = EquationPattern::new_from_symbol_filters(vec![]);
        let mut puzzle_generator = PuzzleGenerator {
            riddle_equation_pattern: equation_pattern,
            number_matchstick_movements: 3,
            solution_equation_pattern: None,
        };

        assert_eq!(3, *puzzle_generator.get_number_matchstick_movements());

        puzzle_generator.set_number_matchstick_movements(1);
        assert_eq!(1, *puzzle_generator.get_number_matchstick_movements());
    }

    #[test]
    fn change_riddle_equation_pattern() {
        let equation_pattern = EquationPattern::new_from_symbol_filters(vec![]);
        let mut puzzle_generator = PuzzleGenerator {
            riddle_equation_pattern: equation_pattern.clone(),
            number_matchstick_movements: 3,
            solution_equation_pattern: None,
        };

        assert_eq!(
            equation_pattern,
            *puzzle_generator.get_riddle_equation_pattern()
        );

        let changed_equation_pattern =
            EquationPattern::new_from_symbol_filters(vec![SymbolFilter::IsAny]);

        puzzle_generator.set_riddle_equation_pattern(changed_equation_pattern.clone());
        assert_eq!(
            changed_equation_pattern,
            *puzzle_generator.get_riddle_equation_pattern()
        );
    }
}
