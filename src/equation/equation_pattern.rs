use super::Equation;
use crate::symbol::SymbolFilter;

use itertools::Itertools;

/// Sequence of [`SymbolFilter`]s generally describing an equation
#[derive(Clone, Debug, PartialEq)]
pub struct EquationPattern {
    pub(super) symbol_filters: Vec<SymbolFilter>,
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
mod test {
    use super::*;
    use crate::symbol::Symbol;

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
