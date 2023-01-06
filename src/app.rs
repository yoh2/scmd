use std::path::Path;

pub fn app_name() -> String {
    let first_arg = std::env::args_os().next().expect("failed to get app name");
    Path::new(&first_arg)
        .file_name()
        .expect("failed to get app name")
        .to_string_lossy()
        .to_string()
}
