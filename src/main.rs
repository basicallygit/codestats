use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::env;

mod map_extension;

#[derive(Default, Debug)]
struct Stats {
    linecount: HashMap<String, usize>,
    filecount: HashMap<String, usize>,
}

impl Stats {
    fn new() -> Stats {
        Default::default()
    }

    #[inline(always)]
    fn add_lines(&mut self, lang: String, lines: usize) {
        let count = self.linecount.entry(lang).or_insert(0);
        *count += lines;
    }

    #[inline(always)]
    fn add_file(&mut self, lang: String) {
        let count = self.filecount.entry(lang).or_insert(0);
        *count += 1;
    }

    fn walk_dir(&mut self, dir: &Path) {
        if let Ok(contents) = dir.read_dir() {
            for entry in contents.map(|e| e.unwrap().path()) {
                if entry.is_dir() {
                    self.walk_dir(&entry);
                }
                else {
                    let mut validutf8 = true;
                    let mut lines = 0;

                    if let Ok(f) = File::open(&entry) {
                        for line in BufReader::new(f).lines() {
                            if let Ok(_) = line {
                                lines += 1;
                            }
                            else {
                                validutf8 = false;
                                break;
                            }
                        }
                    }
                    else {
                        eprintln!("{}: Access denied", entry.display());
                        continue;
                    }

                    if validutf8 {
                        let lang = match entry.extension() {
                            Some(ext) => ext.to_os_string().into_string().unwrap(),
                            None => String::from("?")
                        };

                        self.add_file(lang.clone());
                        self.add_lines(lang, lines);
                    }
                }
            }
        }
        else {
            eprintln!("{}: Access denied", dir.display());
        }
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let path;
    if argc > 1 {
        path = Path::new(&argv[1]);
        if !path.exists() {
            eprintln!("{}: No such file or directory", &argv[1]);
            exit(1);
        }
        else if path.is_file() {
            eprintln!("{}: Item is a file, not a directory", &argv[1]);
            exit(1);
        }
    }
    else {
        path = Path::new(".");
    }

    let mut stats = Stats::new();
    stats.walk_dir(path);

    let extensions = stats.linecount.keys().collect::<Vec<_>>();
    let mut unknownfiles: usize = 0;
    let mut unknownlines: usize = 0;

    println!("EXTENSION\tLINES\t        FILES");
    println!("---------\t-----\t        -----");

    for ext in extensions {
        let mapped = map_extension::map_extension(ext);

        if mapped == "?" {
            unknownlines += stats.linecount[ext];
            unknownfiles += stats.filecount[ext];
        }
        else {
            println!("{:<10}\t{:<8}\t{:<10}", mapped, stats.linecount[ext], stats.filecount[ext]);
        }
    }

    println!("{:<10}\t{:<8}\t{:<10}", "Other", unknownlines, unknownfiles);
}
