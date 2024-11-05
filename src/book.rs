use std::{collections::HashMap, io::{self, Write}};
use regex::Regex;
use colored::Colorize;

use crate::constants;

pub struct Book {
    author: String,
    title: String,
    publisher: String,
    month: String,
    year: i32,
    isbn: String
}

impl Book {
    pub fn new(input: &str) -> Result<Book, String> {
        let re = Regex::new(constants::REGEX_ENTRY).unwrap();
        let mut fields: HashMap<String, String> = HashMap::new();

        for cap in re.captures_iter(input) {
            fields.insert(cap[1].to_string(), cap[2].trim_matches('{').trim_matches('}').to_string());
        }

        let author = fields.get("author").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty author field")?.to_string();
        let title = fields.get("title").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty title field")?.to_string();
        let publisher = fields.get("publisher").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty title field")?.to_string();
        let mut month = fields.get("month").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty month field")?.to_string();
        month.truncate(3);
        let year = fields.get("year").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty year field")?.to_string();
        let isbn = fields.get("isbn").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty isbn field")?.to_string();

        let author_re = Regex::new(constants::REGEX_AUTHOR).unwrap();
        let title_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let publisher_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let month_re = Regex::new(constants::REGEX_MONTH).unwrap();
        let isbn_re = Regex::new(constants::REGEX_ISBN).unwrap();

        if !author_re.is_match(&author) {
            return Err(format!("{} Invalid authors format", constants::ERR.red()));
        }

        if !title_re.is_match(&title) {
            return Err(format!("{} Invalid title format", constants::ERR.red()));
        }

        if !publisher_re.is_match(&publisher) {
            return Err(format!("{} Invalid publisher format", constants::ERR.red()));
        }

        if !month_re.is_match(&month) {
            return Err(format!("{} Invalid month format", constants::ERR.red()));
        }

        if !isbn_re.is_match(&isbn) {
            return Err(format!("{} Invalid ISBN format", constants::ERR.red()));
        }

        let year: i32 = match year.parse() {
            Ok(y) => y,
            Err(_) => return Err(format!("{} Invalid year format", constants::ERR.red())),
        };

        Ok(Book {
            author,
            title,
            publisher,
            month,
            year,
            isbn
        })
    }

    pub fn print<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "@book{{{},", self.generate_key())?;
        writeln!(writer, "    author         = {{{}}},", self.author)?;
        writeln!(writer, "    title          = {{{{{}}}}},", self.title)?;
        writeln!(writer, "    publisher      = {{{{{}}}}},", self.publisher)?;
        writeln!(writer, "    month          = {{{}}},", self.month)?;
        writeln!(writer, "    year           = {{{}}},", self.year)?;
        writeln!(writer, "    isbn           = {},", self.isbn)?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn generate_key(&self) -> String {
        let first_author_last_name = self.author.split(',').next().unwrap_or("").trim();
        format!("{}{}", first_author_last_name.to_lowercase(), self.year)
    }

}