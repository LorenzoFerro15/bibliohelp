pub const REGEX_ENTRY: &str = r"(?m)^\s*(\w+)\s*=\s*(\{{0,2}[^{}]*\}{0,2}|[^,]+),?\s*$"; 

pub const REGEX_AUTHOR: &str = r"(?:[A-Za-z\.'-]+(?: [A-Za-z\.'-]+)?, ?[A-Za-z\.'-]+(?: [A-Za-z\.'-]+)?) (?:and [A-Za-z\.'-]+(?: [A-Za-z\.'-]+)?, ?[A-Za-z\.'-]+(?: [A-Za-z\.'-]+)?)*(?: and [A-Za-z\.'-]+(?: [A-Za-z\.'-]+)?, ?[A-Za-z\.'-]+(?: [A-Za-z\.'-]+)?)?|(?:[A-Za-z\.'-]+(?: [A-Za-z\.'-]+)?, ?[A-Za-z\.'-]+(?: [A-Za-z\.'-]+)?)";
pub const REGEX_TITLE: &str = r"^[a-zA-Z0-9\s,;?!.:()-_]+$";
pub const REGEX_ADDRESS: &str = r"^[A-Za-z\s]+ \([A-Za-z\s]+(?:, [A-Za-z\s]+)?\)$";
pub const REGEX_PAGES: &str = r"^\d+-\d+$";
pub const REGEX_DOI: &str = r"^10\.\d+\/[-._;()\/:A-Za-z0-9\.]+$";
pub const REGEX_MONTH: &str = r"^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)$";
pub const REGEX_ISBN: &str = r"^[\d-]+$";

pub const ERR: &str = "[ERR]";
pub const WARN: &str = "[WARN]";
pub const INFO: &str = "[INFO]";