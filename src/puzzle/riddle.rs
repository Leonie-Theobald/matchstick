use super::SolutionWrapper;
use crate::equation::Equation;

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
    /// # use matchstick::puzzle::riddle::Riddle;
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
    pub(super) fn solve(&self) -> SolutionWrapper {
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
