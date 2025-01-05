use std::{
    fs::File,
    io::{Cursor, Read},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

use compress_tools::uncompress_archive_file;

pub mod calc_set_shared;
pub mod ultimatum;

const NEXT_CHANGE_LINE: &str = "\"next_change_id\"";

fn extract_next_change_id(s: &str) -> Option<&str> {
    if let Some(mut idx) = s.find(NEXT_CHANGE_LINE) {
        idx += NEXT_CHANGE_LINE.len();
        let first_semi = s[idx..].find("\"").unwrap() + idx;
        let second_semi = s[first_semi + 1..].find("\"").unwrap() + first_semi + 1;
        let change_id = &s[first_semi + 1..second_semi];
        Some(change_id)
    } else {
        None
    }
}

trait StashesSource {
    fn next(&mut self) -> Option<String>;
}

pub struct StashesIterator {
    src: Box<dyn StashesSource>,
}

impl Iterator for StashesIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.src.next()
    }
}

pub struct DirStashes {
    path: PathBuf,
    next_change_id: Option<String>,
}

impl DirStashes {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        DirStashes {
            path: path.as_ref().into(),
            next_change_id: Some("index".to_string()),
        }
    }

    pub fn into_iter(self) -> StashesIterator {
        StashesIterator {
            src: Box::new(self),
        }
    }
}

impl StashesSource for DirStashes {
    fn next(&mut self) -> Option<String> {
        if let Some(id) = self.next_change_id.take() {
            let filename = format!("{}.json", id);
            let mut f = match File::open(self.path.join(filename)) {
                Ok(f) => f,
                Err(_) => return None,
            };
            let mut s = String::with_capacity(
                f.metadata()
                    .expect(&format!("cannot read file: {}", id))
                    .size() as usize,
            );
            f.read_to_string(&mut s).expect("cannot read to string");
            let change_id = extract_next_change_id(&s).map(|s| s.to_string());
            self.next_change_id = change_id;
            Some(s)
        } else {
            None
        }
    }
}

pub struct ArchiveStashes {
    data: Vec<u8>,
    next_change_id: Option<String>,
    buf: Vec<u8>,
}

impl ArchiveStashes {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut src = File::open(path).expect("cannot open archive");
        let mut archive = Vec::with_capacity(src.metadata().unwrap().size() as usize);
        src.read_to_end(&mut archive)
            .expect("cannot read archive into memory");
        ArchiveStashes {
            data: archive,
            next_change_id: Some("index".to_string()),
            buf: Vec::with_capacity(10 * 1024 * 1024),
        }
    }

    pub fn into_iter(self) -> StashesIterator {
        StashesIterator {
            src: Box::new(self),
        }
    }
}

impl StashesSource for ArchiveStashes {
    fn next(&mut self) -> Option<String> {
        if let Some(id) = self.next_change_id.take() {
            let filename = format!("{}.json", id);
            self.buf.clear();
            let mut cur = Cursor::new(&self.data);
            let bytes = uncompress_archive_file(&mut cur, &mut self.buf, &filename)
                .expect("cannot read file from archive");
            let s = unsafe { String::from_utf8_unchecked((&self.buf[..bytes]).to_vec()) };
            let change_id = extract_next_change_id(&s).map(|s| s.to_string());
            self.next_change_id = change_id;
            Some(s)
        } else {
            None
        }
    }
}
