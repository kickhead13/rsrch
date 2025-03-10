use crate::output_formatting::format_print;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

mod file_search;
mod automata;
mod output_formatting;
mod cl_handler;

extern crate num_cpus;

use std::fs;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

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
    let automata = automata::Automata::new(pattern.to_string());
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
            let ratio = ((dir_len)/num_cores) + 1;
            
            for _ in 0..(num_cores) {
                let t_automata = automata::Automata::new(pattern.to_string());
                let mut thread_dir_list: Vec<String> = Vec::<String>::new();
                for _ in 0..(ratio) {
                    if dir_list.len() == 0 {
                        break;
                    }
                    thread_dir_list.push(dir_list[dir_list.len()-1].clone());
                    _ = dir_list.pop();
                }

                let pat = pattern.to_string();
                GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
                let _ = std::thread::spawn( move || {
                    for dir in thread_dir_list {
                        file_search::recursion_search(&t_automata, &mode, &dir, &pat);
                    }
                    GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
                });
            }
        }
        if let Some(files) = file_search::list_(
                file_search::FileType::File,
                directory_path
        ) {
            for path in files {
                if &mode == &cl_handler::ExecMode::Name {
                    format_print(&automata, &pattern,&path, &path);
                } else if &mode == &cl_handler::ExecMode::Content {
                    if let Ok(file_content) = fs::read_to_string(
                        path.to_string()
                    ) {
                        format_print(&automata, &pattern,&file_content, &path);
                    }
                }
            }
        }
        while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
            std::thread::sleep(Duration::from_millis(3)); 
        }
        std::process::exit(0);
    }
    file_search::recursion_search(&automata, &mode, &directory_path, &pattern);
}

