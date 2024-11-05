use std::{collections::HashMap, io::{self, Write}};
use regex::Regex;
use colored::Colorize;

use crate::constants;

pub struct Proceedings {
    author: String,
    title: String,
    booktitle: String,
    address: String,
    year: i32,
    month: String,
    pages: String,
    doi: String
}

impl Proceedings {
    pub fn new(input: &str) ->Result<Proceedings, String> {

        let re = Regex::new(constants::REGEX_ENTRY).unwrap();
        let mut fields: HashMap<String, String> = HashMap::new();

        for cap in re.captures_iter(input) {
            fields.insert(cap[1].to_string(), cap[2].trim_matches('{').trim_matches('}').to_string());
        }

        let author = fields.get("author").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty author field")?.to_string();
        let title = fields.get("title").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty title field")?.to_string();
        let booktitle = fields.get("booktitle").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty booktitle field")?.to_string();
        let address = fields.get("address").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty address field")?.to_string();
        let year = fields.get("year").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty year field")?.to_string();
        let mut month = fields.get("month").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty month field")?.to_string();
        month.truncate(3);
        let pages = match fields.get("pages") {
            Some(s) if !s.is_empty() => s.to_string(),
            Some(_) => String::new(),
            None => String::new(),
        };
        let mut doi = match fields.get("doi") {
            Some(s) if !s.is_empty() => s.to_string(),
            Some(_) => String::new(),
            None => return Err("Missing doi field".to_string()),
        };

        let author_re = Regex::new(constants::REGEX_AUTHOR).unwrap();
        let title_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let booktitle_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let address_re = Regex::new(constants::REGEX_ADDRESS).unwrap();
        let pages_re = Regex::new(constants::REGEX_PAGES).unwrap();
        let doi_re = Regex::new(constants::REGEX_DOI).unwrap();
        let month_re = Regex::new(constants::REGEX_MONTH).unwrap();

        if !author_re.is_match(&author) {
            return Err(format!("{} Invalid authors format", constants::ERR.red()));
        }

        if !title_re.is_match(&title) {
            return Err(format!("{} Invalid title format", constants::ERR.red()));
        }

        if !booktitle_re.is_match(&booktitle) {
            return Err(format!("{} Invalid booktitle format", constants::ERR.red()));        }

        if !address_re.is_match(&address) {
            return Err(format!("{} Invalid address format", constants::ERR.red()));        }

        if pages == "" || pages.is_empty(){
            eprintln!("{} Non present page number in entry with title: {}", constants::WARN.yellow(), title);
        } else if !pages_re.is_match(&pages) {
            return Err(format!("{} Invalid pages format: |{}|",constants::ERR.red(), pages));
        }else {
            let pages_split: Vec<&str> = pages.split("-").collect();
            let start_page = pages_split[0].parse::<i32>().unwrap();
            let end_page = pages_split[1].parse::<i32>().unwrap();
            if start_page > end_page {
                return Err(format!("{} Invalid pages second page is lower than first", constants::ERR.red()));
            }
        }

        let year: i32 = match year.parse() {
            Ok(y) => y,
            Err(_) => return Err(format!("{} Invalid year format", constants::ERR.red())),
        };

        if !doi_re.is_match(&doi) {
            eprintln!("{} Invalid DOI format in entry with title: {}", constants::WARN.yellow().yellow(), title);
            doi = String::new();
        }

        if !month_re.is_match(&month) {
            return Err(format!("{} Invalid month format", constants::ERR.red()));
        }

        Ok(Proceedings {
            author,
            title,
            booktitle,
            address,
            year,
            month,
            pages,
            doi
        })
    }

    pub fn print<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "@inproceedings{{{},", self.generate_key())?;
        writeln!(writer, "    author         = {{{}}},", self.author)?;
        writeln!(writer, "    title          = {{{{{}}}}},", self.title)?;
        writeln!(writer, "    booktitle      = {{{{{}}}}},", self.booktitle)?;
        writeln!(writer, "    address        = {{{}}},", self.address)?;
        writeln!(writer, "    year           = {{{}}},", self.year)?;
        writeln!(writer, "    month          = {},", self.month)?;
        writeln!(writer, "    pages          = {{{}}},", self.pages)?;
        writeln!(writer, "    doi            = {{{}}},", self.doi)?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn generate_key(&self) -> String {
        let first_author_last_name = self.author.split(',').next().unwrap_or("").trim();
        format!("{}{}", first_author_last_name.to_lowercase(), self.year)
    }

}