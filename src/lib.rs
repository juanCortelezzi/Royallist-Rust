pub mod path {
    use std::path::Path;
    use termcolor::Color;

    pub fn match_icon(extension: &str) -> (char, Color) {
        match extension {
            "h" => ('', Color::Magenta),
            "c" => ('', Color::Magenta),
            "jpg" => ('', Color::Magenta),
            "png" => ('', Color::Magenta),
            "vim" => ('', Color::Green),
            "zip" => ('', Color::Green),
            "scss" => ('', Color::Red),
            "html" => ('', Color::Red),
            "pdf" => ('', Color::Red),
            "json" => ('', Color::Yellow),
            "yml" => ('', Color::Yellow),
            "yaml" => ('', Color::Yellow),
            "js" => ('', Color::Yellow),
            "jsx" => ('', Color::Yellow),
            "mp4" => ('', Color::Blue),
            "cpp" => ('', Color::Blue),
            "lua" => ('', Color::Blue),
            "py" => ('', Color::Blue),
            "go" => ('ﳑ', Color::Blue),
            "css" => ('', Color::Blue),
            "ts" => ('', Color::Blue),
            "tsx" => ('', Color::Blue),
            "sh" => ('', Color::White),
            "txt" => ('', Color::White),
            "mk" => ('', Color::White),
            "rs" => ('', Color::White),
            "iso" => ('', Color::White),
            "md" => ('', Color::White),
            "mdx" => ('', Color::White),
            "wiki" => ('', Color::White),
            "mod" => ('', Color::White),
            "sum" => ('', Color::White),
            "toml" => ('', Color::White),
            _ => ('', Color::Green),
        }
    }

    pub fn path_filename(path: &Path) -> String {
        match path.file_name() {
            Some(name) => name.to_os_string().into_string().unwrap(),
            None => panic!("couldnt get filename"),
        }
    }

    pub fn path_extension(path: &Path) -> String {
        match path.extension() {
            Some(name) => name.to_os_string().into_string().unwrap(),
            None => String::new(),
        }
    }
}

// var Filetypes = map[string]string{
// 	".h":    magenta(""),
// 	".c":    magenta(""),
// 	".jpg":  magenta(""),
// 	".png":  magenta(""),
// 	".vim":  green(""),
// 	".zip":  green(""),
// 	".scss": red(""),
// 	".html": red(""),
// 	".pdf":  red(""),
// 	".json": yellow(""),
// 	".yml":  yellow(""),
// 	".yaml": yellow(""),
// 	".js":   yellow(""),
// 	".jsx":  yellow(""),
// 	".mp4":  blue(""),
// 	".cpp":  blue(""),
// 	".lua":  blue(""),
// 	".py":   blue(""),
// 	".go":   blue("ﳑ"),
// 	".css":  blue(""),
// 	".ts":   blue(""),
// 	".tsx":  blue(""),
// 	".sh":   white(""),
// 	".txt":  white(""),
// 	".mk":   white(""),
// 	".rs":   white(""),
// 	".iso":  white(""),
// 	".md":   white(""),
// 	".mdx":  white(""),
// 	".wiki": white(""),
// 	".mod":  white(""),
// 	".sum":  white(""),
// 	".toml": white(""),
// }
//
// var Names = map[string]string{
// 	"directory":  cyan(""),
// 	"file":       green(""),
// 	"LICENSE":    white("ﲘ"),
// 	"Makefile":   white(""),
// 	".gitignore": magenta(""),
// }
