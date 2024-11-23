use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum AppMode {
    //通常
    Normal,
    //照会
    Inquery,
    //登録
    Register,
}

impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode_str = match self {
            AppMode::Normal => "Normal",
            AppMode::Inquery => "Inquery",
            AppMode::Register => "Register",
        };
        write!(f, "{}", mode_str)
    }
}
