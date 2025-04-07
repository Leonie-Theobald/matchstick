use crate::equation::Equation;

/// Holds information to describe the solution of a matchstick riddle
#[derive(Debug, PartialEq)]
pub struct Solution {
    solution_equations: Vec<Equation>,
}

impl Solution {
    /// Getter function for riddle [`EquationPattern`]
    pub(super) fn get_solution_equations(&self) -> &Vec<Equation> {
        &self.solution_equations
    }
}

/// Wraps [`Solution`]s of a [`super::Riddle`]
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
    /// # use matchstick::puzzle::riddle::Riddle;
    /// # use matchstick::puzzle::solution::Solution;
    /// # use matchstick::puzzle::solution::SolutionWrapper;
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

    pub(super) fn new_programmatically_set_solution(solution_equations: Vec<Equation>) -> Self {
        SolutionWrapper::ProgrammaticallySet(Solution { solution_equations })
    }

    // unwrap the wrapper
    // This can panic. Only use this if solution is already set
    pub(super) fn get_inner_reference(&self) -> Result<&Solution, ()> {
        match self {
            SolutionWrapper::NotYetSet => Err(()),
            SolutionWrapper::ProgrammaticallySet(solution)
            | SolutionWrapper::ManuallySet(solution) => Ok(solution),
        }
    }
}
