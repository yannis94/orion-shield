use rand::Rng;

struct Password {
    length: u8,
    with_upper_case: bool,
    with_symbol: bool,
}

impl Password {
    fn config(len: u8, upper: bool, sym: bool) -> Password {
        Password {
            length: len,
            with_upper_case: upper,
            with_symbol: sym,
        }
    }

    fn generate(&self) -> String {
        let lower_case_letters = [
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];
        let upper_case_letters = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        ];
        let symbols = [
            "!", "£", "$", "%", "^", "&", "*", "_", "-", "+", "=", "@", "#", "/", "?", "/",
        ];

        let mut char_remaining = self.length;
        let mut rnd = rand::thread_rng();
        let mut symb_nbr: u8 = 0;
        let mut upper_nbr: u8 = 0;
        let mut result: String = String::from("");

        if self.with_symbol {
            symb_nbr = rnd.gen_range(0..char_remaining);
            char_remaining -= symb_nbr;
        }
        if self.with_upper_case {
            upper_nbr = rnd.gen_range(0..char_remaining);
            char_remaining -= upper_nbr;
        }
        let lower_nbr = char_remaining;

        for _i in 0..=symb_nbr {
            let r = rnd.gen_range(0..symbols.len());
            result.push_str(symbols[r])
        }

        for _i in 0..=upper_nbr {
            let r = rnd.gen_range(0..upper_case_letters.len());
            result.push_str(upper_case_letters[r]);
        }

        for _i in 0..=lower_nbr {
            let r = rnd.gen_range(0..lower_case_letters.len());
            result.push_str(lower_case_letters[r]);
        }

        result
    }
}

fn main() {
    println!("Password generator");
    let pwd: Password = Password::config(17, true, true);

    println!("Password > {}", pwd.generate());
    println!("Password > {}", pwd.generate());
    println!("Password > {}", pwd.generate());
}
