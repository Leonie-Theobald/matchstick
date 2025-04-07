use evalexpr::eval_int;
use itertools::Itertools;

use crate::symbol::{Symbol, SymbolFilter};
use crate::transition::TransitionSequence;

/// Holds list of [`Symbol`]s to represent a mathematical equation (or expression)
#[derive(Clone, Debug, PartialEq)]
pub struct Equation {
    symbols: Vec<Symbol>,
}

impl Equation {
    /// Every "character" in an [`Equation`] is represented by a [`Symbol`] \
    /// E.g. 35 - 26 = 9 is built like this:
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::symbol::Symbol;
    /// let equation = Equation::new_from_symbols(vec![
    ///     Symbol::Three,
    ///     Symbol::Five,
    ///     Symbol::Minus,
    ///     Symbol::Two,
    ///     Symbol::Six,
    ///     Symbol::Equal,
    ///     Symbol::Nine,
    /// ]);
    /// ```
    /// An [`Equation`] does not necessarily represent a true mathematical statement.
    /// This is also valid: 2 + 3 = 6
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::symbol::Symbol;
    /// let equation = Equation::new_from_symbols(vec![
    ///     Symbol::Two,
    ///     Symbol::Plus,
    ///     Symbol::Three,
    ///     Symbol::Equal,
    ///     Symbol::Six,
    /// ]);
    /// ```
    pub fn new_from_symbols(symbols: Vec<Symbol>) -> Self {
        Equation { symbols }
    }

    // get all equations that can be formed from matchstick movements
    pub(crate) fn move_n_matchsticks(&self, number_matchsticks: usize) -> Vec<Self> {
        let number_symbols = self.symbols.len();
        let all_transition_sequences =
            TransitionSequence::move_n(number_matchsticks, number_symbols);

        // find all equations that can be formed using the original equation and the transition sequences
        let mut syntactically_correct_equations = Vec::new();
        for transition_sequence in all_transition_sequences {
            if let Ok(mut transitioned_equations) =
                self.apply_transition_sequence(transition_sequence)
            {
                syntactically_correct_equations.append(&mut transitioned_equations);
            }
        }

        syntactically_correct_equations
    }

    pub(crate) fn mathematically_validate(&self) -> Result<(), ()> {
        let equation_string = self.to_plain_text();

        let equation_expressions = equation_string.split("=").map(eval_int).collect::<Vec<_>>();
        if equation_expressions.len() < 2 {
            return Err(());
        } // equation needs at least two expressions
        let Some(Ok(value_first_expression)) = equation_expressions.first() else {
            return Err(());
        }; // there exits at least a first expression

        for expression in &equation_expressions {
            // check that all expressions have same value
            match expression {
                Ok(value) => {
                    if value != value_first_expression {
                        return Err(());
                    }
                }
                Err(_) => return Err(()),
            }
        }

        Ok(())
    }

    pub(crate) fn fulfills_abstract_equation(&self, abstract_equation: &EquationPattern) -> bool {
        for (symbol, allowed_options) in self.symbols.iter().zip(&abstract_equation.symbol_filters)
        {
            if !allowed_options.get_corresponding_symbols().contains(symbol) {
                return false; // found a position where the symbol doesn't fulfill filter options of abstract equation
            }
        }

        true
    }

    pub(crate) fn to_plain_text(&self) -> String {
        let mut string_equation = String::new();
        for symbol in &self.symbols {
            string_equation.push_str(symbol.to_str());
        }
        string_equation
    }

    fn apply_transition_sequence(
        &self,
        transition_sequence: TransitionSequence,
    ) -> Result<Vec<Self>, ()> {
        if self.symbols.len() != transition_sequence.get_number_of_transitions() {
            return Err(());
        } // each symbol of equation needs corresponding transition element

        // apply each transition to respective symbol and collect potential, resulting symbols
        let mut transitioned_symbols = Vec::new();
        for (symbol, transition) in self.symbols.iter().zip(transition_sequence.transitions) {
            // this transition applied to this symbol leads to valid, new symbol(s)
            transitioned_symbols.push(symbol.apply_transition(transition))
        }

        // there are resulting symbols for the first, second, and so on original symbol
        // form each possible equation by applying cartesian product
        // Eg. if original equation consisted of symbols "(A, G, E)" and those symbols transitioned to ((F, T), (H), (N, R, W))
        // all combinations of transitioned equations would be: (F, H, N) and (F, H, R) and (F, H, W) and (T, H, N) and so on
        let all_symbol_combinations = transitioned_symbols
            .into_iter()
            .map(|all_symbol_options| all_symbol_options.into_iter())
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        // put vector of symbols into Equation
        let all_transitioned_equations = all_symbol_combinations
            .into_iter()
            .map(|symbols_of_one_equation| Equation {
                symbols: symbols_of_one_equation,
            })
            .collect();

        Ok(all_transitioned_equations)
    }

    /// Drawing of [`Equation`] to visualize matchstick placement
    /// ```
    /// # use matchstick::equation::Equation;
    /// # use matchstick::symbol::Symbol;
    /// let equation = Equation::new_from_symbols(vec![
    ///     Symbol::Two,
    ///     Symbol::Plus,
    ///     Symbol::Three,
    ///     Symbol::Equal,
    ///     Symbol::Five
    /// ]);
    ///
    /// assert_eq!(equation.draw(),
    #[doc = "\" ___       ___       ___ "]
    #[doc = "    |         | _ _ |    "]
    #[doc = " _ _| _|_  _ _| _ _ |_ _ "]
    #[doc = "|      |      |         |"]
    #[doc = "|___       ___|      ___|\");"]
    /// ```
    pub fn draw(&self) -> String {
        let mut segment_display_lines = vec![String::new(); 5];

        for symbol in &self.symbols {
            symbol
                .draw()
                .split('\n')
                .enumerate()
                .for_each(|(index, line)| {
                    if let Some(string) = segment_display_lines.get_mut(index) {
                        string.push_str(line)
                    }
                });
        }

        segment_display_lines.join("\n")
    }
}

/// Sequence of [`SymbolFilter`]s generally describing an equation
#[derive(Clone, Debug, PartialEq)]
pub struct EquationPattern {
    symbol_filters: Vec<SymbolFilter>,
}

impl EquationPattern {
    /// Create new [`EquationPattern`]
    pub fn new_from_symbol_filters(symbol_filters: Vec<SymbolFilter>) -> Self {
        EquationPattern { symbol_filters }
    }

    pub(crate) fn derive_concrete_equations(&self) -> Vec<Equation> {
        let mut symbols_for_positions = Vec::new();

        // go through each symbol position of the abstract equation
        // and retrieve all allowed symbols for this position
        for filter in &self.symbol_filters {
            let symbols_for_position = filter.get_corresponding_symbols();
            symbols_for_positions.push(symbols_for_position);
        }

        // create all symbol combinations for the equation
        let all_symbol_combinations = symbols_for_positions
            .into_iter()
            .map(|all_symbol_options| all_symbol_options.into_iter())
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        // put vector of symbols into Equation
        all_symbol_combinations
            .into_iter()
            .map(Equation::new_from_symbols)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transition::Transition;

    #[test]
    fn apply_a_transition_sequence() {
        let transition_sequence = TransitionSequence {
            transitions: vec![
                Transition { remove: 0, add: 0 },
                Transition { remove: 1, add: 0 },
                Transition { remove: 0, add: 1 },
                Transition { remove: 0, add: 0 },
                Transition { remove: 0, add: 0 },
            ],
        };
        let equation = Equation {
            symbols: vec![
                Symbol::Seven,
                Symbol::Plus,
                Symbol::OneVar1,
                Symbol::Equal,
                Symbol::Zero,
            ],
        };

        let expected_equation = Equation {
            symbols: vec![
                Symbol::Seven,
                Symbol::Minus,
                Symbol::Seven,
                Symbol::Equal,
                Symbol::Zero,
            ],
        };

        let resulting_equations = equation
            .apply_transition_sequence(transition_sequence)
            .unwrap();
        if resulting_equations.len() != 1 {
            panic!("Should only result in one transitioned equation.")
        }

        assert_eq!(expected_equation, resulting_equations[0])
    }

    #[test]
    fn valid_equation_result_first() {
        let equation = Equation {
            symbols: vec![
                Symbol::Five,
                Symbol::Equal,
                Symbol::Three,
                Symbol::Plus,
                Symbol::Two,
            ],
        };
        assert_eq!(Ok(()), equation.mathematically_validate())
    }

    #[test]
    fn valid_equation_result_second() {
        let equation = Equation {
            symbols: vec![
                Symbol::Five,
                Symbol::Minus,
                Symbol::Three,
                Symbol::Equal,
                Symbol::Two,
            ],
        };
        assert_eq!(Ok(()), equation.mathematically_validate())
    }

    #[test]
    fn valid_equation_result_negative() {
        let equation = Equation {
            symbols: vec![
                Symbol::Five,
                Symbol::Minus,
                Symbol::EightVar1,
                Symbol::Equal,
                Symbol::Minus,
                Symbol::Three,
            ],
        };
        assert_eq!(Ok(()), equation.mathematically_validate())
    }

    #[test]
    fn invalid_equation() {
        let equation = Equation {
            symbols: vec![
                Symbol::Five,
                Symbol::Plus,
                Symbol::EightVar1,
                Symbol::Equal,
                Symbol::Two,
            ],
        };
        assert_eq!(Err(()), equation.mathematically_validate())
    }

    #[test]
    fn no_equal_sign() {
        let equation = Equation {
            symbols: vec![
                Symbol::Three,
                Symbol::Minus,
                Symbol::EightVar1,
                Symbol::Plus,
                Symbol::Seven,
            ],
        };
        assert_eq!(Err(()), equation.mathematically_validate())
    }

    #[test]
    fn two_consecutive_equal_signs() {
        let equation = Equation {
            symbols: vec![
                Symbol::Three,
                Symbol::Plus,
                Symbol::EightVar1,
                Symbol::Equal,
                Symbol::Equal,
                Symbol::Two,
            ],
        };
        assert_eq!(Err(()), equation.mathematically_validate())
    }

    #[test]
    fn two_individual_equal_signs() {
        let equation = Equation {
            symbols: vec![
                Symbol::Two,
                Symbol::Equal,
                Symbol::Two,
                Symbol::Equal,
                Symbol::Two,
            ],
        };
        assert_eq!(Ok(()), equation.mathematically_validate())
    }

    /*
    #[test]
    fn test_create_leading_minus() {
        let equation = Equation {
            symbols: vec![
                Symbol::Nine,
                Symbol::Plus,
                Symbol::OneVar1,
                Symbol::OneVar1,
                Symbol::Equal,
                Symbol::Six,
            ],
        };

        let expected_solved_equations = vec![Equation {
            symbols: vec![
                Symbol::Minus,
                Symbol::Five,
                Symbol::Plus,
                Symbol::OneVar1,
                Symbol::OneVar1,
                Symbol::Equal,
                Symbol::Six,
            ],
        }];

        // TODO: Test case fails because algorithm doesn't consider adding
        // new symbol position in front of equation
        assert_eq!(
            expected_solved_equations,
            equation.move_n_matchsticks_for_valid_equations(1)
        );
    }*/

    #[test]
    fn build_specific_equations() {
        let equation_pattern = EquationPattern {
            symbol_filters: vec![
                SymbolFilter::IsNumber,   // 0 1 2 3 4 5 6 7 8 9
                SymbolFilter::IsOperator, // + - =
            ],
        };

        let expected_specific_equations = vec![
            Equation {
                symbols: vec![Symbol::OneVar1, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::OneVar1, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::OneVar1, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::OneVar2, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::OneVar2, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::OneVar2, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::Two, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::Two, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::Two, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::Three, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::Three, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::Three, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::FourVar1, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::FourVar1, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::FourVar1, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::FourVar2, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::FourVar2, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::FourVar2, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::Five, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::Five, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::Five, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::Six, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::Six, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::Six, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::Seven, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::Seven, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::Seven, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::EightVar1, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::EightVar1, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::EightVar1, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::EightVar2, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::EightVar2, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::EightVar2, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::Nine, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::Nine, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::Nine, Symbol::Equal],
            },
            Equation {
                symbols: vec![Symbol::Zero, Symbol::Minus],
            },
            Equation {
                symbols: vec![Symbol::Zero, Symbol::Plus],
            },
            Equation {
                symbols: vec![Symbol::Zero, Symbol::Equal],
            },
        ];

        assert_eq!(
            expected_specific_equations,
            equation_pattern.derive_concrete_equations()
        )
    }
}
