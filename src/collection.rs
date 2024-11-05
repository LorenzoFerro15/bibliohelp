use std::{collections::HashMap, io::{self, Write}};
use regex::Regex;
use colored::Colorize;

use crate::constants;


pub struct Collection {
    author: String,
    title: String,
    booktitle: String,
    editor: String,
    publisher: String,
    year: i32,
    pages: String,
    isbn: String,
    doi: String
}

impl Collection {
    pub fn new(input: &str) -> Result<Collection, String> {
        let re = Regex::new(constants::REGEX_ENTRY).unwrap();
        let mut fields: HashMap<String, String> = HashMap::new();

        for cap in re.captures_iter(input) {
            fields.insert(cap[1].to_string(), cap[2].trim_matches('{').trim_matches('}').to_string());
        }

        let author = fields.get("author").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty author field")?.to_string();
        let title = fields.get("title").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty title field")?.to_string();
        let booktitle = fields.get("booktitle").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty booktitle field")?.to_string();
        let editor = fields.get("editor").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty editor field")?.to_string();
        let publisher = fields.get("publisher").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty publisher field")?.to_string();
        let year = fields.get("year").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty year field")?.to_string();
        let pages = match fields.get("pages") {
            Some(s) if !s.is_empty() => s.to_string(),
            Some(_) => String::new(),
            None => String::new(),
        };
        let isbn = fields.get("isbn").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty isbn field")?.to_string();
        let doi = fields.get("doi").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty doi field")?.to_string();

        let author_re = Regex::new(constants::REGEX_AUTHOR).unwrap();
        let title_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let booktitle_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let editor_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let publisher_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let pages_re = Regex::new(constants::REGEX_PAGES).unwrap();
        let isbn_re = Regex::new(constants::REGEX_ISBN).unwrap();
        let doi_re = Regex::new(constants::REGEX_DOI).unwrap();

        if !author_re.is_match(&author) {
            return Err(format!("{} Invalid authors format", constants::ERR.red()));
        }

        if !title_re.is_match(&title) {
            return Err(format!("{} Invalid title format", constants::ERR.red()));
        }

        if !booktitle_re.is_match(&booktitle) {
            return Err(format!("{} Invalid booktitle format", constants::ERR.red()));
        }

        if !editor_re.is_match(&editor) {
            return Err(format!("{} Invalid editor format", constants::ERR.red()));
        }

        if !publisher_re.is_match(&publisher) {
            return Err(format!("{} Invalid publisher format", constants::ERR.red()));
        }

        if !pages_re.is_match(&pages) {
            return Err(format!("{} Invalid pages format", constants::ERR.red()));
        }

        if !isbn_re.is_match(&isbn) {
            return Err(format!("{} Invalid ISBN format", constants::ERR.red()));
        }

        if !doi_re.is_match(&doi) {
            return Err(format!("{} Invalid DOI format", constants::ERR.red()));
        }

        let year: i32 = match year.parse() {
            Ok(y) => y,
            Err(_) => return Err(format!("{} Invalid year format", constants::ERR.red())),
        };

        Ok(Collection {
            author,
            title,
            booktitle,
            editor,
            publisher,
            year,
            pages,
            isbn,
            doi
        })
    }

    pub fn print<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "@inproceedings{{{},", self.generate_key())?;
        writeln!(writer, "    author         = {{{}}},", self.author)?;
        writeln!(writer, "    title          = {{{{{}}}}},", self.title)?;
        writeln!(writer, "    booktitle      = {{{{{}}}}},", self.booktitle)?;
        writeln!(writer, "    editor         = {{{}}},", self.editor)?;
        writeln!(writer, "    publisher      = {{{}}},", self.publisher)?;
        writeln!(writer, "    year           = {{{}}},", self.year)?;
        writeln!(writer, "    pages          = {{{}}},", self.pages)?;
        writeln!(writer, "    isbn           = {{{}}},", self.isbn)?;
        writeln!(writer, "    doi            = {{{}}}", self.doi)?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn generate_key(&self) -> String {
        let first_author_last_name = self.author.split(',').next().unwrap_or("").trim();
        format!("{}{}", first_author_last_name.to_lowercase(), self.year)
    }
}