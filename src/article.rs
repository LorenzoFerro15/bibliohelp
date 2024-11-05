use std::{collections::HashMap, io::{self, Write}};
use regex::Regex;

use crate::constants;

pub struct Article {
    author: String,
    title: String,
    journal: String,
    volume: i32,
    number: i32,
    month: String,
    year: i32,
    pages: String,
    doi: String
}

impl Article {
    pub fn new(input: &str ) -> Result<Article, String> {
        let re = Regex::new(constants::REGEX_ENTRY).unwrap();
        let mut fields: HashMap<String, String> = HashMap::new();

        for cap in re.captures_iter(input) {
            fields.insert(cap[1].to_string(), cap[2].trim_matches('{').trim_matches('}').to_string());
        }

        let author = fields.get("author").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty author field")?.to_string();
        let title = fields.get("title").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty title field")?.to_string();
        let journal = fields.get("journal").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty journal field")?.to_string();
        let volume = match fields.get("volume") {
            Some(s) if !s.is_empty() => s.to_string(),
            Some(_) => String::new(),
            None => String::new(),
        };
        let number = match fields.get("number") {
            Some(s) if !s.is_empty() => s.to_string(),
            Some(_) => String::new(),
            None => String::new(),
        };
        let mut month = fields.get("month").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty month field")?.to_string();
        month.truncate(3);
        let year = fields.get("year").and_then(|s| if !s.is_empty() { Some(s) } else { None }).ok_or("Missing or empty year field")?.to_string();
        let pages = match fields.get("pages") {
            Some(s) if !s.is_empty() => s.to_string(),
            Some(_) => String::new(),
            None => String::new(),
        };
        let mut doi = match fields.get("doi") {
            Some(s) if !s.is_empty() => s.to_string(),
            Some(_) => String::new(),
            None => String::new(),
        };

        let author_re = Regex::new(constants::REGEX_AUTHOR).unwrap();
        let title_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let journal_re = Regex::new(constants::REGEX_TITLE).unwrap();
        let month_re = Regex::new(constants::REGEX_MONTH).unwrap();
        let pages_re = Regex::new(constants::REGEX_PAGES).unwrap();
        let doi_re = Regex::new(constants::REGEX_DOI).unwrap();

        if !author_re.is_match(&author) {
            return Err("[ERR] Invalid authors format".to_string());
        }

        if !title_re.is_match(&title) {
            return Err("[ERR] Invalid title format".to_string());
        }

        if !journal_re.is_match(&journal) {
            return Err("[ERR] Invalid journal format".to_string());
        }

        if !month_re.is_match(&month) {
            return Err("[ERR] Invalid month format".to_string());
        }

        if !pages_re.is_match(&pages) {
            return Err("[ERR] Invalid pages format".to_string());
        }

        if pages == "" || pages.is_empty() {
            eprintln!("[WARN] Non present page number in entry with title: {}", title);
        } else if !pages_re.is_match(&pages) {
            return Err(format!("[ERR] Invalid pages format: |{}|", pages));
        }else {
            let pages_split: Vec<&str> = pages.split("-").collect();
            let start_page = pages_split[0].parse::<i32>().unwrap();
            let end_page = pages_split[1].parse::<i32>().unwrap();
            if start_page > end_page {
                return Err("[ERR] Invalid pages second page is lower than first".to_string());
            }
        }

        let year: i32 = match year.parse() {
            Ok(y) => y,
            Err(_) => return Err("[ERR] Invalid year format".to_string()),
        };

        let volume: i32 = match volume.parse() {
            Ok(y) => y,
            Err(_) => {
                eprintln!("[WARN] Invalid volume format in entry with title: {}", title);
                -1
            }
            
        };

        let number: i32 = match number.parse() {
            Ok(y) => y,
            Err(_) => {
                eprintln!("[WARN] Invalid number format in entry with title: {}", title);
                -1
            }
        };

        if !doi_re.is_match(&doi) {
            eprintln!("[WARN] Invalid DOI format in entry with title: {}", title);
            doi = String::new();
        }

        Ok(Article {
            author,
            title,
            journal,
            volume,
            number,
            month,
            year,
            pages,
            doi
        })
    }

    pub fn print<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "@article{{{},", self.generate_key())?;
        writeln!(writer, "    author         = {{{}}},", self.author)?;
        writeln!(writer, "    title          = {{{{{}}}}},", self.title)?;
        writeln!(writer, "    journal        = {{{{{}}}}},", self.journal)?;
        if self.volume != -1 {
            writeln!(writer, "    volume         = {{{}}},", self.volume)?;
        } else {
            writeln!(writer, "    volume         = {{}}")?;
        }
        if self.number != -1 {
            writeln!(writer, "    number         = {{{}}},", self.number)?;
        } else {
            writeln!(writer, "    number         = {{}}")?;
        }
        writeln!(writer, "    month          = {},", self.month)?;
        writeln!(writer, "    year           = {{{}}},", self.year)?;
        writeln!(writer, "    pages          = {{{}}}", self.pages)?;
        writeln!(writer, "    doi            = {{{}}}", self.doi)?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn generate_key(&self) -> String {
        let first_author_last_name = self.author.split(',').next().unwrap_or("").trim();
        format!("{}{}", first_author_last_name.to_lowercase(), self.year)
    }
}