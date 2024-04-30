use std::fs;
use crate::cl_handler::ExecMode;
use crate::format_print;

#[derive(PartialEq)]
pub enum FileType {
    Directory,
    File
}

pub fn list_(mode: FileType, path: String) -> Option<Vec<String>> {
    use FileType::File;

    let mut bvalue = true;
    if mode == File {
        bvalue = false;
    }
    let paths = fs::read_dir(path).unwrap();
    let mut result_vector: Vec<_> = Vec::<String>::new();
    for path in paths {
        let is_path_dir = fs::metadata(
               path.as_ref()
                .unwrap()
                .path()
                .display()
                .to_string()
            )
            .unwrap()
            .is_dir();
        if is_path_dir == bvalue {
            result_vector.push(
                path.unwrap()
                .path()
                .display()
                .to_string()
            );
        }
    }
    if result_vector.len() > 0 {
        return Some(result_vector);
    }
    return None;
}

pub fn recursion_search(mode: &ExecMode, path: &String, pattern: &String) -> () {
    use FileType::{File,Directory};

    if let Some(file_paths) = list_(File, path.to_string()) {
        for path in file_paths.to_vec().iter() {
            if mode == &ExecMode::Name  {
                format_print(&pattern, &path, &path);
            } else if mode == &ExecMode::Content {
                if let Ok(file_content) = fs::read_to_string(path) {
                    format_print(&pattern, &file_content, &path);
                }
            }
        }
    }

    if let Some(dir_paths) = list_(Directory, path.to_string()) {

        for path in dir_paths.to_vec().iter() {
            if mode == &ExecMode::Name  {
                format_print(&pattern, &path, &path);
            }
            recursion_search(mode, path, pattern);
        }
    }
}


