mod automata;
mod output_formatting;
use crate::output_formatting::format_print;

mod file_search;

mod cl_handler;

extern crate num_cpus;

use std::fs;

#[allow(unused_variables)]
#[allow(unreachable_code)]
fn main() {

    let (mode, pattern, directory_path, help_mess, parallelize) =
        cl_handler::handle_cl_args(std::env::args()
            .collect::<Vec<String>>()
            .to_vec()
            .iter()
            .enumerate()
            .filter_map(|(i,e)| 
                if i != 0 {
                    Some(e.to_string())
                } else {
                    None
                }
            )
            .collect()
        );
    
    if help_mess == true {
        cl_handler::print_help();
        std::process::exit(0);
    } 
    if parallelize {
        if let Some(mut dir_list) = file_search::list_(
                file_search::FileType::Directory,
                directory_path.to_string()
        ) {
            let num_cores = num_cpus::get();
            let dir_len = dir_list.len();
            let mut last_end = 0;
            for iter in 0..(num_cores-1) {
                let ratio = (dir_len-1)/num_cores;
                let slice_start = last_end;
                let slice_end = 
                    std::cmp::min(
                        slice_start + ratio + 1,
                        dir_len-1
                    );
                let thread_dir_list: Vec<String> = 
                    dir_list[slice_start..slice_end]
                    .to_vec()
                    .clone();
                let pat = pattern.to_string();
                let _ = std::thread::spawn( move || {
                    for dir in thread_dir_list {
                        file_search::recursion_search(&mode, &dir, &pat);
                    }
                }).join();
                last_end = slice_end;
            }
            let slice_end = 
                std::cmp::min(
                    last_end + ((dir_len-1)/num_cores)+1,
                    dir_len-1
                );
            dir_list = dir_list[last_end..slice_end]
                .to_vec()
                .clone();
            for dir in dir_list {
                file_search::recursion_search(
                    &mode,
                    &directory_path,
                    &pattern
                );
            }
        }
        if let Some(files) = file_search::list_(
                file_search::FileType::File,
                directory_path
        ) {
            for path in files {
                if &mode == &cl_handler::ExecMode::Name {
                    format_print(&pattern,&path, &path);
                } else if &mode == &cl_handler::ExecMode::Content {
                    if let Ok(file_content) = fs::read_to_string(
                        path.to_string()
                    ) {
                        format_print(&pattern,&file_content, &path);
                    }
                }
            }
        }
        std::process::exit(0);
    }
    file_search::recursion_search(&mode, &directory_path, &pattern);
}

