
#[cfg(test)]
mod tests{
    use std::ops::Not;
    use std::str::Chars;
    #[test]
    fn test1(){
        let token: String = String::from("что-тоfだ🤣染め");
        for c in token.char_indices(){
            println!("index: {}, char: {}",c.0, c.1)
        }
        let emoji_bytes: Vec<u8> = token.as_bytes()[15..19].to_vec();
        print!("[");
        emoji_bytes.iter().for_each(|x|{print!("{x},")});
        print!("]\n");
        println!("char at index 14: {}", str::from_utf8(&emoji_bytes).unwrap())
    }

    #[test]
    fn bool_test(){
        let mut some_bool: bool = false;

        for _ in 1..3u8 {
            some_bool=some_bool.not();
            println!("state: {some_bool}")
        }
    }

    #[test]
    fn stdout_test(){
        use std::io::{stdout, Write};

        stdout().write_all("ж".as_bytes()).unwrap();
    }

    #[test]
    fn quotes_test(){
        let c = "\'";
        println!("{c}")
    }
    #[test]
    fn enumerate_test(){
        let string = "привет world";
        for (i,c) in string.chars().enumerate(){
            println!("{}: {}",i,c);
        }
    }
}