#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_types2() {
        let my_first_initial = 'C';

        if my_first_initial.is_alphabetic() {
            println!("Alphabetical!");
        } 
        else if my_first_initial.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }

        let your_character = '7';
        if your_character.is_alphabetic() {
            println!("Alphabetical!");
        } else if your_character.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }
    }
}
