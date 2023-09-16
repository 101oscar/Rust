#[cfg(test)]

mod tests {

    #[test]
    fn test_primitive_types1() {
        let is_morning = true;
        if is_morning {
            println!("Good morning!");
        } 
        else {
            let is_evening = false;
            if is_evening {
                println!("Good evening!");
            }
        }
    }
}
