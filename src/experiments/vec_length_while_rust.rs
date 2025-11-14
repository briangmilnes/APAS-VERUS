pub fn vec_length_while(s: &Vec<usize>) -> usize {
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    while i < s.len() {
        i += 1;
        length += 1;
    }
    length
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let len = vec_length_while(&v);
    println!("Length: {}", len);
}

