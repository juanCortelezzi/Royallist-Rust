use termcolor::Color::{self, Blue, Cyan, Green, Magenta, Red, White, Yellow};

pub type Icon = (char, Color);

pub fn match_ext_icon(extension: &str) -> Icon {
    match extension {
        "h" => ('', Magenta),
        "c" => ('', Magenta),
        "jpg" => ('', Magenta),
        "png" => ('', Magenta),
        "vim" => ('', Green),
        "zip" => ('', Green),
        "scss" => ('', Red),
        "html" => ('', Red),
        "pdf" => ('', Red),
        "json" => ('', Yellow),
        "yml" => ('', Yellow),
        "yaml" => ('', Yellow),
        "js" => ('', Yellow),
        "jsx" => ('', Yellow),
        "mp4" => ('', Blue),
        "cpp" => ('', Blue),
        "lua" => ('', Blue),
        "py" => ('', Blue),
        "go" => ('ﳑ', Blue),
        "css" => ('', Blue),
        "ts" => ('', Blue),
        "tsx" => ('', Blue),
        "sh" => ('', White),
        "txt" => ('', White),
        "mk" => ('', White),
        "rs" => ('', White),
        "iso" => ('', White),
        "md" => ('', White),
        "mdx" => ('', White),
        "wiki" => ('', White),
        "mod" => ('', White),
        "sum" => ('', White),
        "toml" => ('', White),
        "lock" => ('', White),
        _ => ('', Green),
    }
}

pub fn match_dir_icon(name: &str) -> (char, Color) {
    match name {
        ".git" => ('', Cyan),
        "node_modules" => ('', Cyan),
        _ => ('', Cyan),
    }
}

// var Names = map[string]string{
// 	"directory":  cyan(""),
// 	"file":       green(""),
// 	"LICENSE":    white("ﲘ"),
// 	"Makefile":   white(""),
// 	".gitignore": magenta(""),
// }
