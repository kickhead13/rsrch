use crate::automata::Automata;

pub fn format_print(pattern: &String, text: &String, path: &String) {
    if let Some((vec, _)) = Automata::new(pattern.to_string()).eval(text.to_string()) {
        print!("found in {} at ", path.to_string());
        for pos in vec {
            print!("{pos} ");
        }
        println!();
    }
}
