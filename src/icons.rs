use termcolor::Color;

pub type Icon = (char, Color);

pub fn match_ext_icon(extension: &str) -> Icon {
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

pub fn match_dir_icon(name: &str) -> (char, Color) {
    match name {
        ".git" => ('', Color::Cyan),
        "node_modules" => ('', Color::Cyan),
        _ => ('', Color::Cyan),
    }
}

// var Names = map[string]string{
// 	"directory":  cyan(""),
// 	"file":       green(""),
// 	"LICENSE":    white("ﲘ"),
// 	"Makefile":   white(""),
// 	".gitignore": magenta(""),
// }
