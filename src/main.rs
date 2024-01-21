mod automata;

mod output_formatting;

mod file_search;

mod cl_handler;

extern crate num_cpus;

use std::fs;

#[allow(unused_variables)]
#[allow(unreachable_code)]
fn main() {

    let (mode, pattern, directory_path, help_mess, parallelize) =
        cl_handler::handle(std::env::args()
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
        println!("main lol");
        cl_handler::print_help();
        std::process::exit(0);
    }
    

    if parallelize {
        if let Some(mut dir_list) =
            file_search::list_(file_search::FileType::Directory, directory_path.to_string())
        {
            let num_cores = num_cpus::get();
            for iter in 0..(dir_len) {
                let dir = dir_list[dir_len-iter-1].to_string();
                dir_list.pop();
                let pat = pattern.to_string();
                let _ = std::thread::spawn( move || {
                    file_search::recursion_search(&mode, &dir, &pat);
                }).join();
            }
        }
        if let Some(files) =
            file_search::list_(file_search::FileType::File, directory_path) 
        {
            for path in files {
                if &mode == &cl_handler::ExecMode::Name {
                    output_formatting::format_print_content(
                        &pattern,
                        &path,
                        &path
                    );
                } else if &mode == &cl_handler::ExecMode::Content {
                    if let Ok(file_content) = fs::read_to_string(path.to_string()) {
                        output_formatting::format_print_content(
                            &pattern, 
                            &file_content,
                            &path
                        );
                    }
                }
            }
        }
    } else {
        file_search::recursion_search(&mode, &directory_path, &pattern);
    }
}

