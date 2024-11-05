use std::fs::File;
use std::io::Read;
use std::env;

mod proceedings;
mod article;
mod constants;
mod book;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    
    let mut file = File::open(file_name)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    
    let _ = extract_entries(&buffer);

    Ok(())
}

fn extract_entries(input: &str) -> std::io::Result<()> {
    let mut iter = input.chars().peekable();
    let mut entry_content = String::new();
    let mut brace_count;
    let mut entry_type = String::new();

    let mut output_file = File::create("biblio.bib")?;

    while let Some(c) = iter.next() {
        if c == '@' {
            entry_type.clear();
            entry_content.clear();
            brace_count = 0;

            while let Some(&next_char) = iter.peek() {
                if next_char == '{' {
                    iter.next();
                    brace_count += 1;
                    break;
                }
                entry_type.push(next_char);
                iter.next();
            }

            while let Some(&next_char) = iter.peek() {
                if next_char == '{' {
                    brace_count += 1;
                } else if next_char == '}' {
                    brace_count -= 1;
                    if brace_count == 0 {
                        iter.next();
                        break;
                    }
                }
                entry_content.push(next_char);
                iter.next();
            }

            let entry_id: String = entry_content.chars().take_while(|&c| c != ',').collect();

            match entry_type.trim() {
                "inproceedings" => {
                    match proceedings::Proceedings::new(&entry_content) {
                        Ok(proceeding) => {
                            proceeding.print(&mut output_file).unwrap();
                        }
                        Err(e) => {
                            eprintln!("[ERR] Error processing inproceedings entry {}: {}", entry_id, e);
                        }
                    }
                }

                "article" => {
                    match article::Article::new(&entry_content) {
                        Ok(article) => {
                            article.print(&mut output_file).unwrap();
                        }
                        Err(e) => {
                            eprintln!("[ERR] Error processing article entry {}: {}", entry_id, e);
                        }
                    }
                }
                "book" => {
                    match book::Book::new(&entry_content) {
                        Ok(book) => {
                            book.print(&mut output_file).unwrap();
                        }
                        Err(e) => {
                            eprintln!("[ERR] Error processing book entry {}: {}", entry_id, e);
                        }
                    }
                }
                "misc" => {
                }
                _ => {
                    eprintln!("Unknown entry type: {}", entry_type);
                }
            }
        }
    }

    Ok(())
}