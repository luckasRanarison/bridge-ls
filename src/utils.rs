use std::path::Path;

pub fn expand_args(args: &[String], file_path: &Path) -> Vec<String> {
    args.iter()
        .map(|arg| match arg.as_str() {
            "$filepath" => file_path
                .to_str()
                .map(|path| path.to_owned())
                .unwrap_or_default(),
            "$filename" => file_path
                .file_name()
                .and_then(|f| f.to_str().map(|f| f.to_owned()))
                .unwrap_or_default(),
            _ => arg.to_owned(),
        })
        .collect()
}
