static ALPHABET_LEN: usize = 128;

pub struct Automata {   
    pub pattern_len: usize,
    pub delta: Vec<Vec<usize>>
}

impl Automata {
    pub fn vecn() -> Vec<usize> {
        let mut to_be_returned = Vec::<usize>::new();
        for _ in 0..ALPHABET_LEN {
            to_be_returned.push(0);
        }
        to_be_returned
    }
   
    pub fn sufix(pattern: &String, prefix: &String) -> usize {
        let prefix_bytes = prefix.as_bytes();
        let mut sufixed: String = String::new();
        for iter in 0..(prefix_bytes.len()) {
            for byte in &prefix_bytes[iter..] {
                sufixed.push(*byte as char);
            }
            if sufixed.len() != 0 && pattern.starts_with(&sufixed) {
                return sufixed.len();
            }
            sufixed = String::new();
       }
        return 0;
    }

    pub fn new(pattern: String) -> Self {
        let mut delta_create = Vec::new();
        let pattern_len = pattern.len();
        let mut prefix_string: String = String::new();
        let pattern_bytes = pattern.as_bytes();
        for row_no in 0..=pattern_len {
            let mut row: Vec<usize> = Self::vecn();
            for iter in 0..ALPHABET_LEN {
               if row_no != pattern_len && iter as u8 as char == pattern_bytes[row_no] as char {
                    row[iter] = row_no+1;
                } else {
                    prefix_string.push(iter as u8 as char);
                    row[iter] = Self::sufix(&pattern, &prefix_string);
                    prefix_string.pop();
                }
            }
            if row_no < pattern_len {
                prefix_string.push(pattern_bytes[row_no] as char);
            }
            delta_create.push(row.clone());
        }
        Self {
            pattern_len: pattern.len(),
            delta: delta_create
        }
    }
    
    pub fn eval(self, text: String) -> Option<(Vec<usize>,usize)> {
        let text_bytes: Vec<u8> = text.as_bytes().to_vec();
        let mut state: usize = 0;
        let mut result_vector: Vec<usize> = Vec::<usize>::new();
        for (iter,val) in text_bytes.iter().enumerate() {
            if (*val) as usize <= ALPHABET_LEN-1 {
                state = self.delta[state][(*val) as usize];
                if state == self.pattern_len {
                    result_vector.push((iter as i32 - self.pattern_len as i32 + 1) as usize);
                }
            } else {
                state = 0;
            }
        }
        if result_vector.len() > 0 {
            return Some((result_vector, self.pattern_len));
        }
        return None;
    }
    
    #[allow(unused_variables)]
    #[allow(dead_code)]
    pub fn format(path: String, vect: Vec<usize>, pat_len: usize) -> String {
        todo!()
    }
}
