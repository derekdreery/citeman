use error_gen::ErrorGen;
use std::{io, path::PathBuf, str::FromStr};
use uuid::Uuid;

mod crossref;

/// The generic document record
#[derive(Debug)]
pub struct Document {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<Author>,
    pub doc_type: DocumentType,
    pub year: Option<u32>,
    pub paper_location: PathBuf,
}

#[derive(Debug)]
pub enum DocumentType {
    Journal {
        volume: Option<String>,
        number: Option<String>,
        pages: Option<Pages>,
    },
    Book {
        publisher: String,
    },
}

#[derive(Debug)]
pub struct Author {
    pub given_names: String,
    pub family_name: String,
}

impl FromStr for Author {
    type Err = InvalidAuthor;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(if input.contains(',') {
            let mut iter = input.splitn(2, ',');
            let family_name = iter.next().ok_or(InvalidAuthor)?;
            let given_names = iter.next().ok_or(InvalidAuthor)?;
            if family_name.trim().is_empty() || given_names.trim().is_empty() {
                return Err(InvalidAuthor);
            }
            Author {
                given_names: given_names.trim().to_owned(),
                family_name: family_name.trim().to_owned(),
            }
        } else {
            let mut iter = input.rsplitn(2, ' ');
            let family_name = iter.next().ok_or(InvalidAuthor)?;
            let given_names = iter.next().ok_or(InvalidAuthor)?;
            if family_name.trim().is_empty() || given_names.trim().is_empty() {
                return Err(InvalidAuthor);
            }
            Author {
                given_names: given_names.trim().to_owned(),
                family_name: family_name.trim().to_owned(),
            }
        })
    }
}

#[derive(Debug, ErrorGen)]
pub struct InvalidAuthor;

#[derive(Debug)]
pub enum Pages {
    Single(u32),
    Multi { from: u32, to: u32 },
}

impl FromStr for Pages {
    type Err = InvalidPages;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input
            .split(|ch: char| !ch.is_ascii_digit())
            .filter(|s| !s.is_empty());
        let from = iter
            .next()
            .ok_or(InvalidPages)?
            .parse::<u32>()
            .map_err(|_| InvalidPages)?;
        Ok(match iter.next() {
            Some(n) => {
                if let Some(_) = iter.next() {
                    return Err(InvalidPages);
                }
                let to = n.parse::<u32>().map_err(|_| InvalidPages)?;
                Pages::Multi { from, to }
            }
            None => Pages::Single(from),
        })
    }
}

#[derive(ErrorGen, Debug)]
pub struct InvalidPages;

#[derive(Debug)]
/// A document before it is stored in the database.
pub struct NewDocument {
    pub title: String,
    pub authors: Vec<Author>,
    pub doc_type: DocumentType,
    pub year: Option<u32>,
    pub paper_location: PathBuf,
}

impl NewDocument {
    pub fn from_stdin_interactive(path: impl Into<PathBuf>) -> io::Result<Self> {
        let title = read_human::read_string_nonempty("Please enter the title of the document")?;
        let mut authors: Vec<Author> = vec![read_human::read_custom_nonempty(
            "Please enter the authors, leave a blank line at end",
        )?];
        loop {
            let next_author = read_human::read_custom_noquestion()?;
            if let Some(author) = next_author {
                authors.push(author);
            } else {
                break;
            }
        }
        let doc_type = read_human::read_choice(
            "What type of document are you adding",
            &["journal article", "book"],
            Some(0),
        )?;
        let doc_type = match doc_type {
            0 => {
                let volume = read_human::read_string("Volume (optional)")?;
                let number = read_human::read_string("Number (optional)")?;
                let pages: Option<Pages> = read_human::read_custom("Page numbers (optional)")?;
                DocumentType::Journal {
                    volume,
                    number,
                    pages,
                }
            }
            1 => unimplemented!(),
            _ => unreachable!(),
        };
        let year: Option<u32> = read_human::read_custom("Year (optional)")?;
        Ok(NewDocument {
            title,
            authors,
            doc_type,
            year,
            paper_location: path.into(),
        })
    }

    pub fn save(self) -> Document {
        unimplemented!()
    }
}
