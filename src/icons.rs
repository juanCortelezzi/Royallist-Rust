use termcolor::Color::{self, Blue, Cyan, Green, Magenta, Red, White, Yellow};

pub type Icon = (char, Color);

pub fn match_ext_icon(extension: &str) -> Option<Icon> {
    match extension {
        "h" => Some(('', Magenta)),
        "c" => Some(('', Magenta)),
        "jpg" => Some(('', Magenta)),
        "png" => Some(('', Magenta)),
        "vim" => Some(('', Green)),
        "zip" => Some(('', Green)),
        "scss" => Some(('', Red)),
        "html" => Some(('', Red)),
        "pdf" => Some(('', Red)),
        "json" => Some(('', Yellow)),
        "yml" => Some(('', Yellow)),
        "yaml" => Some(('', Yellow)),
        "js" => Some(('', Yellow)),
        "jsx" => Some(('', Yellow)),
        "mp4" => Some(('', Blue)),
        "cpp" => Some(('', Blue)),
        "lua" => Some(('', Blue)),
        "py" => Some(('', Blue)),
        "go" => Some(('ﳑ', Blue)),
        "css" => Some(('', Blue)),
        "ts" => Some(('', Blue)),
        "tsx" => Some(('', Blue)),
        "sh" => Some(('', White)),
        "txt" => Some(('', White)),
        "mk" => Some(('', White)),
        "rs" => Some(('', White)),
        "iso" => Some(('', White)),
        "md" => Some(('', White)),
        "mdx" => Some(('', White)),
        "wiki" => Some(('', White)),
        "mod" => Some(('', White)),
        "sum" => Some(('', White)),
        "toml" => Some(('', White)),
        "lock" => Some(('', White)),
        _ => None,
    }
}

pub fn match_name_icon(name: &str) -> Option<Icon> {
    match name {
        "LICENSE" => Some(('ﲘ', White)),
        "Makefile" => Some(('', White)),
        ".gitignore" => Some(('', Magenta)),
        _ => None,
    }
}

pub fn match_dir_icon(name: &str) -> Option<Icon> {
    match name {
        ".git" => Some(('', Cyan)),
        "node_modules" => Some(('', Cyan)),
        _ => None,
    }
}

pub enum PathType {
    File,
    Dir,
}

pub fn get_default_icon(path_type: PathType) -> Icon {
    match path_type {
        PathType::File => ('', Green),
        PathType::Dir => ('', Cyan),
    }
}

// var Names = map[string]string{
// 	"directory":  cyan(""),
// 	"file":       green(""),
// 	"LICENSE":    white("ﲘ"),
// 	"Makefile":   white(""),
// 	".gitignore": magenta(""),
// }
