use std::collections::HashMap;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::process::exit;

#[derive(Debug)]
struct Stats {
    line_count: HashMap<String, usize>, // <lang, count>
    file_count: HashMap<String, usize>, // <lang, count>
}

impl Stats {
    fn new() -> Self {
        Self {line_count: HashMap::new(), file_count: HashMap::new()}
    }

    fn add_line(&mut self, lang: String) {
        let count = self.line_count.entry(lang).or_insert(0);
        *count += 1;
    }

    fn add_file(&mut self, lang: String) {
        let count = self.file_count.entry(lang).or_insert(0);
        *count += 1;
    }

    fn crawl_dir(&mut self, dir: &Path) {
        for entry in dir.read_dir().unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                self.crawl_dir(&path);
            } else {
                let lang;
                if let Some(ext) = path.extension() {
                    if let Some(ext) = ext.to_str() {
                        lang = ext.to_string();
                    } else {
                        lang = "?".to_string();
                    }
                } else {
                    lang = "?".to_string();
                }
                self.add_file(lang.clone());
                let reader = BufReader::new(File::open(path).unwrap());

                for line in reader.lines() {
                    if let Ok(line) = line {
                        if !line.is_empty() {
                            self.add_line(lang.clone());
                        }
                    }
                }
            }
        }
    }
}

fn convert_extension(ext: &str) -> &'static str {
    match ext {
        "rs" => "Rust",
        "py" => "Python",
        "pyw" => "Python",
        "js" => "JavaScript",
        "html" => "HTML",
        "css" => "CSS",
        "sh" => "Shell",
        "c" => "C",
        "s" => "Assembly",
        "asm" => "Assembly",
        "cxx" => "C++",
        "cpp" => "C++",
        "h" => "C Header",
        "hpp" => "C++ Header",
        "java" => "Java",
        "kt" => "Kotlin",
        "go" => "Go",
        "php" => "PHP",
        "rb" => "Ruby",
        "swift" => "Swift",
        "vb" => "VisualBasic",
        "vbs" => "VBScript",
        "cs" => "C#",
        "pl" => "Perl",
        "lua" => "Lua",
        "hs" => "Haskell",
        "r" => "R",
        "el" => "Emacs Lisp",
        "clj" => "Clojure",
        "cljc" => "Clojure",
        "cljs" => "ClojureScript",
        "ts" => "TypeScript",
        "dart" => "Dart",
        "scala" => "Scala",
        "ml" => "OCaml",
        "elm" => "Elm",
        "erl" => "Erlang",
        "m" => "Objective-C",
        "mm" => "Objective-C++",
        "tex" => "LaTeX",
        "json" => "JSON",
        "xml" => "XML",
        "yml" => "YAML",
        "toml" => "TOML",
        "md" => "Markdown",
        "txt" => "Txt",
        "log" => "Log",
        "jpg" => "JPEG",
        "jpeg" => "JPEG",
        "png" => "PNG",
        "gif" => "GIF",
        "svg" => "SVG",
        "mp3" => "MP3",
        "mp4" => "MP4",
        "mkv" => "MKV",
        "avi" => "AVI",
        "mov" => "MOV",
        "flv" => "FLV",
        "wmv" => "WMV",
        "zip" => "ZIP",
        "zig" => "Zig",
        "tar" => "TAR",
        "gz" => "Gzip",
        "xz" => "Xzip",
        "7z" => "7-Zip",
        "rar" => "RAR",
        "pdf" => "PDF",
        "doc" => "Word",
        "docx" => "Word",
        "xls" => "Excel",
        "xlsx" => "Excel",
        "ppt" => "PowerPoint",
        "pptx" => "PowerPoint",
        "exe" => "Executable",
        "dll" => "DLL",
        "so" => "Shared Object",
        "o" => "Object",
        "a" => "StaticLibrary",
        "lib" => "StaticLibrary",
        "class" => "JavaClass",
        "jar" => "JavaArchive",
        _ => "?"
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut stats = Stats::new();
    let argc = argv.len();
    let path;
    if argc > 1 {
        if argv[1] == "--raw" {
            path = Path::new(".");
        }
        else {
            path = Path::new(&argv[1]);
        }
        if !path.exists() {
            eprintln!("Path does not exist: {}", &argv[1]);
            exit(1);
        }
        else if path.is_file() {
            eprintln!("'{}' is a file, not a directory", &argv[1]);
            exit(1);
        }
        stats.crawl_dir(Path::new(path));
    } else {
        stats.crawl_dir(Path::new("."));
    }

    if argv.contains(&String::from("--raw")) {

        let mut extensions = stats.line_count.keys().collect::<Vec<_>>();

        extensions.sort();

        println!("EXTENSION\tLINES\t        FILES");
        for ext in extensions {
            println!("{:<10}\t{:<8}\t{:<10}", ext, stats.line_count[ext], stats.file_count[ext]);
        }
    }
    else {
        let extensions = stats.line_count.keys().collect::<Vec<_>>();
        let mut unknown_lines = 0;
        let mut unknown_files = 0;

        println!("EXTENSION\tLINES\t        FILES");
        println!("---------\t-----\t        -----");
        for ext in extensions {
            if convert_extension(ext) == "?" {
                unknown_lines += stats.line_count[ext];
                unknown_files += stats.file_count[ext];
            }
            else {
                println!("{:<10}\t{:<8}\t{:<10}", convert_extension(ext), stats.line_count[ext], stats.file_count[ext]);
            }
        }
        println!("{:<10}\t{:<8}\t{:<10}", "Other", unknown_lines, unknown_files);
    }
}