#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum ExecMode {
    Name,
    Content,
    Error
}

pub fn print_help() -> () {
    println!("Usage: rsrch name [PATTERN] [OPTIONS]");
    println!("       rsrch content [PATTERN] [OPTIONS]");
    println!("For \'name\' mode: print path to all files that contain \'PATTERN\'");
    println!("in their relative path; for \'content\' mode: print path to all");
    println!("files that contain \'PATTERN\' within them.");
    println!();
    println!("OPTIONS:");
    println!("   -h,  -help                              display this message");
    println!("   -d,  --directory  <DIRECTORY_PATH=./>   sets the directory within");
    println!("                                           the program will search;");
    println!("                                           current directory by");
    println!("                                           default");
    println!("   -p,  --parallelize                      parallelize file search");
    println!();
    println!("EXIT STATUS:");
    println!("0 if OK");
    println!("1 if search mode is incorrect");
}

pub fn handle_cl_args(args: Vec<String>) -> (ExecMode, String, String, bool, bool) {
    let mut help_mess = false;
    let mut mode: ExecMode = ExecMode::Name;
    let mut pattern_found = false;
    let mut pattern: String = "aaa".to_string();
    let mut last_arg: String = "".to_string();
    let mut directory_path: String = "./".to_string();
    let mut parallelize: bool = false;
    for (pos, arg) in args.iter().enumerate() {
        if pos == 0 {
            if arg == "content" {
                mode = ExecMode::Content;
            } else {
                mode = ExecMode::Name;
            }
        } else {
            if arg == "-h" || arg == "--help" {
                help_mess = true;
            } else if arg == "-p" || arg == "--parallelize" {
                parallelize = true;
            } else if last_arg == "-d" || last_arg == "--directory" {
                directory_path = arg.to_string();
            } else if pattern_found == false && arg.to_string().starts_with("-") == false {
                pattern_found = true;
                pattern = arg.to_string();
            } else if arg != "-d" && arg != "--directory" {
                return (ExecMode::Error, "".to_string(), "".to_string(), true, true);
            }
        }
        last_arg = arg.to_string();
    }
    return (mode, pattern, directory_path, help_mess, parallelize);
}



