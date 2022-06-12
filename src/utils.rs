use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum RadioOption {
    Standard,
    Billed,
    Revert,
}

impl Default for RadioOption {
    fn default() -> Self {
        Self::Standard
    }
}

impl RadioOption {
    pub fn message(&self) -> String {
        match self {
            RadioOption::Standard => "Escriba la entrada de dinero".to_string(),
            RadioOption::Billed => "¿Cuánto dinero necesitas?".to_string(),
            RadioOption::Revert => "¿De cuánto fue el depósito?".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Step {
    Start,
    Calc,
    Save,
}

impl Default for Step {
    fn default() -> Self {
        Self::Start
    }
}

#[cfg(target_os = "linux")]
pub fn assets_location() -> PathBuf {
    PathBuf::from("/usr/share/dwarf")
}

/// Return the execution dir
#[cfg(not(target_os = "linux"))]
pub fn assets_location() -> PathBuf {
    env::current_exe().unwrap()
}
