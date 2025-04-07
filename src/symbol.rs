use crate::segment_display::SegmentDisplay;
use crate::transition::Transition;

/// Filters for [`Symbol`]s with specific characteristics, such as being a number or an operator
#[derive(Clone, Debug, PartialEq)]
pub enum SymbolFilter {
    IsAny,
    IsNumber,
    IsOperator,
    List(Vec<Symbol>),
}

impl SymbolFilter {
    pub(crate) fn get_corresponding_symbols(&self) -> Vec<Symbol> {
        match self {
            SymbolFilter::IsAny => Symbol::get_all(),
            SymbolFilter::List(symbols) => symbols.clone(),
            SymbolFilter::IsNumber | SymbolFilter::IsOperator => {
                let mut list_symbols = Vec::new();
                for symbol in Symbol::get_all() {
                    let symbol_filter_type = match symbol {
                        Symbol::Minus => SymbolFilter::IsOperator,
                        Symbol::Plus => SymbolFilter::IsOperator,
                        Symbol::Equal => SymbolFilter::IsOperator,
                        Symbol::OneVar1 => SymbolFilter::IsNumber,
                        Symbol::OneVar2 => SymbolFilter::IsNumber,
                        Symbol::Two => SymbolFilter::IsNumber,
                        Symbol::Three => SymbolFilter::IsNumber,
                        Symbol::FourVar1 => SymbolFilter::IsNumber,
                        Symbol::FourVar2 => SymbolFilter::IsNumber,
                        Symbol::Five => SymbolFilter::IsNumber,
                        Symbol::Six => SymbolFilter::IsNumber,
                        Symbol::Seven => SymbolFilter::IsNumber,
                        Symbol::EightVar1 => SymbolFilter::IsNumber,
                        Symbol::EightVar2 => SymbolFilter::IsNumber,
                        Symbol::Nine => SymbolFilter::IsNumber,
                        Symbol::Zero => SymbolFilter::IsNumber,
                    };
                    if self == &symbol_filter_type {
                        list_symbols.push(symbol);
                    }
                }
                list_symbols
            }
        }
    }
}

macro_rules! impl_symbols {
    ($($variant:ident $display_string:expr, $drawn_string:expr, ($($position:ident: $value:expr, )*),)*) => {
        /// A specific, meaningful constellation of matchsticks
        /// showing a mathematical "character", like a digit or an operator
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Symbol {
            $(
                #[doc = concat!("The mathematical value is \"", $display_string, "\" and the segment representation is\n```text\n", $drawn_string)]
                $variant,
            )*
        }

        impl Symbol {
            /// All available [`Symbol`] variants
            pub fn get_all() -> Vec<Self> {
                vec![
                    $(
                        Symbol::$variant,
                    )*
                ]
            }

            /// A textual representation of the mathematical symbol
            /// ```
            /// # use matchstick::symbol::Symbol;
            /// let plus = Symbol::Plus;
            /// assert_eq!("+", plus.to_str());
            /// ```
            pub fn to_str(&self) -> &str {
                match self {
                    $(
                        Symbol::$variant => $display_string,
                    )*
                }
            }

            /// Display [`Symbol`] in matchstick layout
            /// ```
            /// # use matchstick::symbol::Symbol;
            /// let seven = Symbol::Seven;
            /// let visual_seven = seven.draw();
            /// assert_eq!(visual_seven,
            #[doc = "\" ___ "] // use doc to prevent removal of trailing whitespace with cargo fmt
            ///     |
            ///     |
            ///     |
            ///     |");
            /// ```
            pub fn draw(&self) -> String {
                let segment_display = self.to_segment_display();
                segment_display.draw()
            }

            fn to_segment_display(&self) -> SegmentDisplay {
                match self {
                    $(
                        Symbol::$variant => SegmentDisplay {
                            $(
                                $position: $value,
                            )*
                        },
                    )*
                }
            }

            // Gets all valid [`Symbol`]s that emerged when adding/removing segments
            // of the original [`Symbol`] according to given matchstick movements
            // E.g. when adding one segment to Five, it results in either Six or Nine
            //            ___
            //           |    __________
            //           |_ _          |
            //  ________     |         |
            //  |         ___|         |
            //  |     ___      ___     |
            //  |    |        |   | <--|
            //  |    |_ _     |_ _|
            //  |--> |   |        |
            //       |___|     ___|
            pub(crate) fn apply_transition(&self, transition: Transition) -> Vec<Self> {
                let mut collected_symbols = Vec::new();
                let segment_display_source_symbol = self.to_segment_display();

                $(  // compare source symbol with all Symbol variants and check for desired transition
                    let segment_display_target_symbol = Self::$variant.to_segment_display();
                    let current_transition = segment_display_source_symbol.delta_to(&segment_display_target_symbol);
                    if current_transition == transition {
                        collected_symbols.push(Self::$variant);
                    }
                )*

                collected_symbols
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            $(
                // validate that segment display in documentation matches
                // segment display given
                #[test]
                #[allow(non_snake_case)]
                fn $variant() {
                    let symbol = Symbol::$variant;
                    assert_eq!(symbol.draw(), $drawn_string);
                }
            )*

        }
    };
}

impl_symbols!(
    Minus "-", "     \n     \n _ _ \n     \n     ", (
        top: false,
        upper_left: false,
        upper_right: false,
        middle_beam: true,
        upper_beam: false,
        pipe: false,
        lower_left: false,
        lower_right: false,
        bottom: false,
    ),
    Plus "+", "     \n     \n _|_ \n  |  \n     ", (
        top: false,
        upper_left: false,
        upper_right: false,
        middle_beam: true,
        upper_beam: false,
        pipe: true,
        lower_left: false,
        lower_right: false,
        bottom: false,
    ),
    Equal "=", "     \n _ _ \n _ _ \n     \n     ", (
        top: false,
        upper_left: false,
        upper_right: false,
        middle_beam: true,
        upper_beam: true,
        pipe: false,
        lower_left: false,
        lower_right: false,
        bottom: false,
    ),
    OneVar1 "1", "     \n    |\n    |\n    |\n    |", (
        top: false,
        upper_left: false,
        upper_right: true,
        middle_beam: false,
        upper_beam: false,
        pipe: false,
        lower_left: false,
        lower_right: true,
        bottom: false,
    ),
    OneVar2 "1", "     \n|    \n|    \n|    \n|    ", (
        top: false,
        upper_left: true,
        upper_right: false,
        middle_beam: false,
        upper_beam: false,
        pipe: false,
        lower_left: true,
        lower_right: false,
        bottom: false,
    ),
    Two "2", " ___ \n    |\n _ _|\n|    \n|___ ", (
        top: true,
        upper_left: false,
        upper_right: true,
        middle_beam: true,
        upper_beam: false,
        pipe: false,
        lower_left: true,
        lower_right: false,
        bottom: true,
    ),
    Three "3", " ___ \n    |\n _ _|\n    |\n ___|", (
        top: true,
        upper_left: false,
        upper_right: true,
        middle_beam: true,
        upper_beam: false,
        pipe: false,
        lower_left: false,
        lower_right: true,
        bottom: true,
    ),
    FourVar1 "4", "     \n|   |\n|_ _|\n    |\n    |", (
        top: false,
        upper_left: true,
        upper_right: true,
        middle_beam: true,
        upper_beam: false,
        pipe: false,
        lower_left: false,
        lower_right: true,
        bottom: false,
    ),
    FourVar2 "4", "     \n|    \n|_|_ \n  |  \n     ", (
        top: false,
        upper_left: true,
        upper_right: false,
        middle_beam: true,
        upper_beam: false,
        pipe: true,
        lower_left: false,
        lower_right: false,
        bottom: false,
    ),
    Five "5", " ___ \n|    \n|_ _ \n    |\n ___|", (
        top: true,
        upper_left: true,
        upper_right: false,
        middle_beam: true,
        upper_beam: false,
        pipe: false,
        lower_left: false,
        lower_right: true,
        bottom: true,
    ),
    Six "6", " ___ \n|    \n|_ _ \n|   |\n|___|", (
        top: true,
        upper_left: true,
        upper_right: false,
        middle_beam: true,
        upper_beam: false,
        pipe: false,
        lower_left: true,
        lower_right: true,
        bottom: true,
    ),
    Seven "7", " ___ \n    |\n    |\n    |\n    |", (
        top: true,
        upper_left: false,
        upper_right: true,
        middle_beam: false,
        upper_beam: false,
        pipe: false,
        lower_left: false,
        lower_right: true,
        bottom: false,
    ),
    EightVar1 "8", " ___ \n|   |\n|_ _|\n|   |\n|___|", (
        top: true,
        upper_left: true,
        upper_right: true,
        middle_beam: true,
        upper_beam: false,
        pipe: false,
        lower_left: true,
        lower_right: true,
        bottom: true,
    ),
    EightVar2 "8", " ___ \n|_ _|\n|_ _|\n     \n     ", (
        top: true,
        upper_left: true,
        upper_right: true,
        middle_beam: true,
        upper_beam: true,
        pipe: false,
        lower_left: false,
        lower_right: false,
        bottom: false,
    ),
    Nine "9", " ___ \n|   |\n|_ _|\n    |\n ___|", (
        top: true,
        upper_left: true,
        upper_right: true,
        middle_beam: true,
        upper_beam: false,
        pipe: false,
        lower_left: false,
        lower_right: true,
        bottom: true,
    ),
    Zero "0", " ___ \n|   |\n|   |\n|   |\n|___|", (
        top: true,
        upper_left: true,
        upper_right: true,
        middle_beam: false,
        upper_beam: false,
        pipe: false,
        lower_left: true,
        lower_right: true,
        bottom: true,
    ),
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_numbers_symbols() {
        let number_symbols = vec![
            Symbol::OneVar1,
            Symbol::OneVar2,
            Symbol::Two,
            Symbol::Three,
            Symbol::FourVar1,
            Symbol::FourVar2,
            Symbol::Five,
            Symbol::Six,
            Symbol::Seven,
            Symbol::EightVar1,
            Symbol::EightVar2,
            Symbol::Nine,
            Symbol::Zero,
        ];

        assert_eq!(
            number_symbols,
            SymbolFilter::IsNumber.get_corresponding_symbols()
        );
    }

    #[test]
    fn get_no_numbers_symbols() {
        let no_number_symbols = vec![Symbol::Minus, Symbol::Plus, Symbol::Equal];

        assert_eq!(
            no_number_symbols,
            SymbolFilter::IsOperator.get_corresponding_symbols()
        );
    }

    #[test]
    fn symbol_filter_no_restrictions() {
        assert_eq!(
            SymbolFilter::IsAny.get_corresponding_symbols(),
            Symbol::get_all()
        );
    }
}
