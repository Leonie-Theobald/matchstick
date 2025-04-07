use matchstick::equation::{Equation, EquationPattern};
use matchstick::puzzle::{Puzzle, PuzzleGenerator, Riddle};
use matchstick::symbol::{Symbol, SymbolFilter};

fn main() {
    /*
    let equation = Equation {
        symbols_for_position: vec![
            Symbol::Minus,
            Symbol::Plus,
            Symbol::Equal,
            Symbol::Zero,
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
        ],
    };
    println!("{}", equation.to_segment_displays());
    */

    let riddle_equation = Equation::new_from_symbols(vec![
        Symbol::Two,
        Symbol::Minus,
        Symbol::Seven,
        Symbol::Equal,
        Symbol::Three,
    ]);
    let mut my_puzzle = Puzzle::new_from_riddle(Riddle::new(riddle_equation, 1));
    my_puzzle.search_and_set_solution();

    println!("My puzzle: {:#?}", my_puzzle);

    let riddle_equation_pattern = EquationPattern::new_from_symbol_filters(vec![
        SymbolFilter::IsNumber,
        SymbolFilter::IsOperator,
        SymbolFilter::IsNumber,
        SymbolFilter::List(vec![Symbol::Equal]),
        SymbolFilter::IsNumber,
    ]);
    let my_puzzle_generator = PuzzleGenerator::new(riddle_equation_pattern, 2);
    let generated_puzzles = my_puzzle_generator.derive_puzzles_with_n_solutions(1);

    println!("Generated puzzles: {:#?}", generated_puzzles);
}
