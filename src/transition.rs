// Information on delta between two elements, e.g. [`crate::segment_display::SegmentDisplay`]s
// or [`crate::symbol::Symbol`]s
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Transition {
    pub remove: usize,
    pub add: usize,
}

impl Transition {
    // transition for single element
    fn remove_one(&mut self) {
        self.remove += 1;
    }

    fn add_one(&mut self) {
        self.add += 1;
    }
}

// Information on delta for several element-pairs, e.g. an [`crate::equation::Equation`]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TransitionSequence {
    pub transitions: Vec<Transition>,
}

impl TransitionSequence {
    fn with_n_default_transitions(n: usize) -> Self {
        let default_transitions = vec![Transition::default(); n];
        TransitionSequence {
            transitions: default_transitions,
        }
    }

    // How many deltas or how many element-pairs are covered
    pub(crate) fn get_number_of_transitions(&self) -> usize {
        self.transitions.len()
    }

    fn move_one(&self) -> Vec<Self> {
        let mut new_transition_sequences = Vec::new();
        for source_index in 0..self.get_number_of_transitions() {
            for target_index in 0..self.get_number_of_transitions() {
                // source index is for where the moved element is coming from
                // target index is for where the moved element is going to
                // source and target index may be equal => moving element within symbol
                let mut new_transition_sequence = self.clone();
                new_transition_sequence.transitions[source_index].remove_one();
                new_transition_sequence.transitions[target_index].add_one();

                new_transition_sequences.push(new_transition_sequence);
            }
        }
        new_transition_sequences
    }

    // Generates all movement pattern that can be formed by number of movements.
    // This is independent of the concrete nature of the elements that will be changed
    // according to the [`Transition`]s.
    pub(crate) fn move_n(number_movements: usize, number_elements: usize) -> Vec<Self> {
        // prepare recursive function call
        let default_transition_sequence = vec![Self::with_n_default_transitions(number_elements)];
        Self::move_n_recursive(number_movements, default_transition_sequence)
    }

    fn move_n_recursive(number_movements: usize, transition_sequences: Vec<Self>) -> Vec<Self> {
        if number_movements == 0 {
            // no movements allowed anymore
            // recursion is finished
            return transition_sequences;
        }

        let mut new_transition_sequences = Vec::new();
        for sequence in transition_sequences {
            new_transition_sequences.append(&mut sequence.move_one());
        }

        // start recursion with one less movement left
        Self::move_n_recursive(number_movements - 1, new_transition_sequences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn move_one_in_one_symbol_from_default() {
        let default_transition_sequence = TransitionSequence::with_n_default_transitions(1);
        let expected_transition_sequence = vec![TransitionSequence {
            transitions: vec![Transition { remove: 1, add: 1 }],
        }];

        assert_eq!(
            expected_transition_sequence,
            default_transition_sequence.move_one()
        );
    }

    #[test]
    fn move_one_in_two_symbols_from_default() {
        let default_transition_sequence = TransitionSequence::with_n_default_transitions(2);
        let expected_transition_sequences = vec![
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 1 },
                    Transition { remove: 0, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 0 },
                    Transition { remove: 0, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 1 },
                    Transition { remove: 1, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 1, add: 1 },
                ],
            },
        ];

        assert_eq!(
            expected_transition_sequences,
            default_transition_sequence.move_one()
        );
    }

    #[test]
    fn move_one_in_three_symbols_from_default() {
        let default_transition_sequence = TransitionSequence::with_n_default_transitions(3);
        let expected_transition_sequence = vec![
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 1 },
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 0, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 0 },
                    Transition { remove: 0, add: 1 },
                    Transition { remove: 0, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 0 },
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 0, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 1 },
                    Transition { remove: 1, add: 0 },
                    Transition { remove: 0, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 1, add: 1 },
                    Transition { remove: 0, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 1, add: 0 },
                    Transition { remove: 0, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 1 },
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 1, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 0, add: 1 },
                    Transition { remove: 1, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 1, add: 1 },
                ],
            },
        ];

        assert_eq!(
            expected_transition_sequence,
            default_transition_sequence.move_one()
        );
    }

    #[test]
    fn move_one_in_one_symbol_from_non_default() {
        let transition_sequence = TransitionSequence {
            transitions: vec![Transition { remove: 5, add: 2 }],
        };
        let expected_transition_sequence = vec![TransitionSequence {
            transitions: vec![Transition { remove: 6, add: 3 }],
        }];

        assert_eq!(expected_transition_sequence, transition_sequence.move_one());
    }

    #[test]
    fn move_two_in_two_symbols_from_default() {
        let default_transition_sequence = vec![TransitionSequence::with_n_default_transitions(2)];

        let expected_transition_sequences = vec![
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 2, add: 2 },
                    Transition { remove: 0, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 2, add: 1 },
                    Transition { remove: 0, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 2 },
                    Transition { remove: 1, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 1 },
                    Transition { remove: 1, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 2, add: 1 },
                    Transition { remove: 0, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 2, add: 0 },
                    Transition { remove: 0, add: 2 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 1 },
                    Transition { remove: 1, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 0 },
                    Transition { remove: 1, add: 2 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 2 },
                    Transition { remove: 1, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 1 },
                    Transition { remove: 1, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 2 },
                    Transition { remove: 2, add: 0 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 1 },
                    Transition { remove: 2, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 1 },
                    Transition { remove: 1, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 1, add: 0 },
                    Transition { remove: 1, add: 2 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 1 },
                    Transition { remove: 2, add: 1 },
                ],
            },
            TransitionSequence {
                transitions: vec![
                    Transition { remove: 0, add: 0 },
                    Transition { remove: 2, add: 2 },
                ],
            },
        ];

        assert_eq!(
            expected_transition_sequences,
            TransitionSequence::move_n_recursive(2, default_transition_sequence)
        );
    }
}
