use crate::transition::Transition;

macro_rules! delta_for_segment_display {
    ($($position:ident,)*) => {
        /// A representation of a digital display used for numbers \
        /// Similar to a 7-segment display
        /// A segment can light up (```true```) or be turned of (```false```).
        pub struct SegmentDisplay {
            $(
                pub $position: bool,
            )*
        }

        impl SegmentDisplay {
            // Shows how many segments need to change state (s. [`Transition`])
            // to reach another [`SegmentDisplay`]
            pub(crate) fn delta_to(&self, target: &Self) -> Transition {
                let mut remove = 0;
                let mut add = 0;

                $(
                    if self.$position != target.$position {
                        match target.$position {
                            true => add += 1,
                            false => remove += 1,
                        }
                    }
                )*

                Transition { remove, add }
            }
        }
    };
}
delta_for_segment_display!(
    top,
    upper_left,
    upper_right,
    upper_beam,
    middle_beam,
    pipe,
    lower_left,
    lower_right,
    bottom,
);

impl SegmentDisplay {
    /// The segments can be visualized with five string lines
    /// ```text
    /// 1  ___
    /// 2 |_ _|
    /// 3 |_|_|
    /// 4 | | |
    /// 5 |___|
    /// ```
    pub fn draw(&self) -> String {
        let mut segment_display = String::new();

        // first line
        if self.top {
            segment_display.push_str(" ___ ");
        } else {
            segment_display.push_str("     ");
        }
        segment_display.push('\n');

        // second line
        if self.upper_left {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }
        if self.upper_beam {
            segment_display.push_str("_ _");
        } else {
            segment_display.push_str("   ");
        }
        if self.upper_right {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }
        segment_display.push('\n');

        // third line
        if self.upper_left {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }
        if self.middle_beam {
            segment_display.push('_');
        } else {
            segment_display.push(' ');
        }
        if self.pipe {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }
        if self.middle_beam {
            segment_display.push('_');
        } else {
            segment_display.push(' ');
        }
        if self.upper_right {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }
        segment_display.push('\n');

        // forth line
        if self.lower_left {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }
        if self.pipe {
            segment_display.push_str(" | ");
        } else {
            segment_display.push_str("   ");
        }
        if self.lower_right {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }
        segment_display.push('\n');

        // fifth line
        if self.lower_left {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }
        if self.bottom {
            segment_display.push_str("___");
        } else {
            segment_display.push_str("   ");
        }
        if self.lower_right {
            segment_display.push('|');
        } else {
            segment_display.push(' ');
        }

        segment_display
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_no_to_all_segments() {
        let start = SegmentDisplay {
            top: false,
            upper_left: false,
            upper_right: false,
            upper_beam: false,
            middle_beam: false,
            pipe: false,
            lower_left: false,
            lower_right: false,
            bottom: false,
        };

        let target = SegmentDisplay {
            top: true,
            upper_left: true,
            upper_right: true,
            upper_beam: true,
            middle_beam: true,
            pipe: true,
            lower_left: true,
            lower_right: true,
            bottom: true,
        };

        let expected_transition = Transition { remove: 0, add: 9 };

        assert_eq!(expected_transition, start.delta_to(&target));
    }

    #[test]
    fn transition_all_to_no_segments() {
        let start = SegmentDisplay {
            top: true,
            upper_left: true,
            upper_right: true,
            upper_beam: true,
            middle_beam: true,
            pipe: true,
            lower_left: true,
            lower_right: true,
            bottom: true,
        };

        let target = SegmentDisplay {
            top: false,
            upper_left: false,
            upper_right: false,
            upper_beam: false,
            middle_beam: false,
            pipe: false,
            lower_left: false,
            lower_right: false,
            bottom: false,
        };

        let expected_transition = Transition { remove: 9, add: 0 };

        assert_eq!(expected_transition, start.delta_to(&target));
    }

    #[test]
    fn transition_mixed_segments() {
        let start = SegmentDisplay {
            top: false,
            upper_left: true,
            upper_right: true,
            upper_beam: false,
            middle_beam: false,
            pipe: false,
            lower_left: false,
            lower_right: true,
            bottom: false,
        };

        let target = SegmentDisplay {
            top: false,
            upper_left: true,
            upper_right: false,
            upper_beam: false,
            middle_beam: true,
            pipe: true,
            lower_left: false,
            lower_right: true,
            bottom: false,
        };

        let expected_transition = Transition { remove: 1, add: 2 };

        assert_eq!(expected_transition, start.delta_to(&target));
    }

    #[test]
    fn draw_no_segment() {
        let segment_display = SegmentDisplay {
            top: false,
            upper_left: false,
            upper_right: false,
            upper_beam: false,
            middle_beam: false,
            pipe: false,
            lower_left: false,
            lower_right: false,
            bottom: false,
        };

        let expected_string = "     
     
     
     
     ";
        assert_eq!(segment_display.draw(), expected_string);
    }

    #[test]
    fn draw_all_segments() {
        let segment_display = SegmentDisplay {
            top: true,
            upper_left: true,
            upper_right: true,
            upper_beam: true,
            middle_beam: true,
            pipe: true,
            lower_left: true,
            lower_right: true,
            bottom: true,
        };

        let expected_string = " ___ 
|_ _|
|_|_|
| | |
|___|";
        assert_eq!(segment_display.draw(), expected_string);
    }

    #[test]
    fn draw_plus() {
        let segment_display = SegmentDisplay {
            top: false,
            upper_left: false,
            upper_right: false,
            upper_beam: false,
            middle_beam: true,
            pipe: true,
            lower_left: false,
            lower_right: false,
            bottom: false,
        };

        let expected_string = "     
     
 _|_ 
  |  
     ";
        assert_eq!(segment_display.draw(), expected_string);
    }
}
