pub mod puzzle_generator;
pub mod riddle;
pub mod solution;

use riddle::Riddle;
use solution::SolutionWrapper;

/// Holds the [`Riddle`] and the [`SolutionWrapper`] containing the [`Solution`]
#[derive(Debug, PartialEq)]
pub struct Puzzle {
    riddle: Riddle,
    wrapped_solution: SolutionWrapper,
}

impl Puzzle {
    /// Creates new [`Puzzle`] from riddle while solution is unknown yet.
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::puzzle::Puzzle;
    /// # use matchstick::puzzle::riddle::Riddle;
    /// # use matchstick::puzzle::solution::SolutionWrapper;
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
            wrapped_solution: SolutionWrapper::NotYetSet,
        }
    }

    /// Programmatically find solution in form of [`Equation`]s fitting to the [`Riddle`] of this [`Puzzle`]\
    /// The found solution is set\
    /// Returns number of found solution [`Equation`]s
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::puzzle::Puzzle;
    /// # use matchstick::puzzle::riddle::Riddle;
    /// # use matchstick::puzzle::solution::SolutionWrapper;
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
        self.wrapped_solution = self.riddle.solve();
        match &self.wrapped_solution {
            SolutionWrapper::NotYetSet => panic!("fn solve() should have just set the solutions"),
            SolutionWrapper::ProgrammaticallySet(solution)
            | SolutionWrapper::ManuallySet(solution) => solution.get_solution_equations().len(),
        }
    }

    /// The [`Solution`] of the [`Puzzle`] is set to given, arbitrary value
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::puzzle::Puzzle;
    /// # use matchstick::puzzle::riddle::Riddle;
    /// # use matchstick::puzzle::solution::Solution;
    /// # use matchstick::puzzle::solution::SolutionWrapper;
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
        self.wrapped_solution = wrapped_solution;
    }

    /// Getter function for [`Riddle`]  
    pub fn get_riddle(&self) -> &Riddle {
        &self.riddle
    }

    /// Getter function for solution wrapped in [`SolutionWrapper`]  
    pub fn get_wrapped_solution(&self) -> &SolutionWrapper {
        &self.wrapped_solution
    }

    /// Setter function for [`Riddle`]  
    pub fn set_riddle(&mut self, riddle: Riddle) {
        self.riddle = riddle
    }
}

#[cfg(test)]
mod test {
    use crate::equation::Equation;
    use crate::symbol::Symbol;

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
            riddle: Riddle::new(riddle_equation.clone(), 2),
            wrapped_solution: SolutionWrapper::NotYetSet,
        };

        let expected_puzzle = Puzzle {
            riddle: Riddle::new(riddle_equation.clone(), 4),
            wrapped_solution: SolutionWrapper::NotYetSet,
        };

        puzzle.set_riddle(Riddle::new(riddle_equation, 4));
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
            riddle: Riddle::new(riddle_equation, 2),
            wrapped_solution: SolutionWrapper::NotYetSet,
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
        assert_eq!(expected_solution, puzzle.wrapped_solution);
    }
}
