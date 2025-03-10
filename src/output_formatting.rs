use crate::automata::Automata;

pub fn format_print(automata: &Automata, _: &String, text: &String, path: &String) {
    if let Some((vec, _)) = automata.clone().eval(text.to_string()) {
        print!("found in {} at ", path.to_string());
        for pos in vec {
            print!("{pos} ");
        }
        println!();
    }
}
