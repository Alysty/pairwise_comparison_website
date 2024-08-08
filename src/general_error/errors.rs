use core::fmt;
use error_stack::Context;

#[derive(Debug)]
pub struct GeneralReportError;

impl fmt::Display for GeneralReportError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("")
    }
}

impl Context for GeneralReportError {}


