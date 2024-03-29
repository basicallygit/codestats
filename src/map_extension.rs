#[inline]
pub fn map_extension(ext: &str) -> &'static str {
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
        "hxx" => "C++ Header",
        "hc" => "HolyC",
        "HC" => "HolyC",
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
        "?" => "Binary/No ext",
        _ => "?",
    }
}
