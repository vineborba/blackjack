use std::{fmt, str::FromStr};

use strum::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum Action {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Surrender,
}

impl Action {
    pub fn can_execute(
        &self,
        hand_size: usize,
        hands_count: usize,
        pot: u32,
        bet: u32,
        cards_are_equal: bool,
    ) -> bool {
        match self {
            Action::DoubleDown => hand_size == 2 && pot >= (bet * 2),
            Action::Split => hand_size == 2 && cards_are_equal && pot >= (bet * 2),
            Action::Surrender => hands_count == 1 && hand_size == 2,
            Action::Hit | Action::Stand => true,
        }
    }

    pub fn print_actions_list(actions: &[Self]) {
        let string_actions = actions
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", string_actions);
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Hit => write!(f, "(H) Hit"),
            Action::Stand => write!(f, "(S) Stand"),
            Action::DoubleDown => write!(f, "(D) Double Down"),
            Action::Split => write!(f, "(X) Split"),
            Action::Surrender => write!(f, "(Q) Surrender"),
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "s" | "S" => Action::Stand,
            "h" | "H" => Action::Hit,
            "d" | "D" => Action::DoubleDown,
            "x" | "X" => Action::Split,
            "q" | "Q" => Action::Surrender,
            _ => return Err("Invalid input for action".into()),
        };
        Ok(action)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[case::hit_lowercase("h", Action::Hit)]
    #[case::hit_uppercase("H", Action::Hit)]
    #[case::stand_lowercase("s", Action::Stand)]
    #[case::stand_uppercase("S", Action::Stand)]
    #[case::double_down_lowercase("d", Action::DoubleDown)]
    #[case::double_down_uppercase("D", Action::DoubleDown)]
    #[case::split_lowercase("x", Action::Split)]
    #[case::split_uppercase("X", Action::Split)]
    #[case::surrender_lowercase("q", Action::Surrender)]
    #[case::surrender_uppercase("Q", Action::Surrender)]
    fn from_str_conversion(#[case] input: &str, #[case] output: Action) {
        assert_eq!(Action::from_str(input).unwrap(), output);
    }

    #[rstest]
    fn can_splt() {}
}
