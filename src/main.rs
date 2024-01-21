mod automata;

mod file_search;

mod cl_handler;

#[allow(unused_variables)]
#[allow(unreachable_code)]
fn main() {

    let (mode, pattern, directory_path, help_mess) =
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
    
    file_search::recursion_search(&mode, &directory_path, &pattern);
}

