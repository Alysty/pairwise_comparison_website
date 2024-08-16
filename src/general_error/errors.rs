use core::fmt;
use error_stack::{Report, Context};

#[derive(Debug)]
pub struct GeneralReportError;

impl fmt::Display for GeneralReportError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("")
    }
}

impl Context for GeneralReportError {}



impl GeneralReportError {
    pub fn from_err<E: ToString>(err: E) -> Report<GeneralReportError> {
        Report::new(GeneralReportError{}).attach_printable(err.to_string())
    }
}
