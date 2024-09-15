mod terminal_intercation {
    // used to handle user actions
    enum Command {
        Config,
        Generate,
        Quit,
    }

    pub fn print_menu() {
        print!("ORION SHIELD");
        print!("Password generator");
    }

    pub fn print_config() {
        print!("should print config");
    }
}

pub mod generator {
    use rand::{thread_rng, Rng};

    use crate::utils;

    pub struct Password {
        length: u8,
        with_upper_case: bool,
        with_spec_char: bool,
        with_digit: bool,
    }

    // (lenght, got upper case, got special char, got digit
    pub type PasswordConfig = (u8, bool, bool, bool);

    impl Password {
        pub fn new(cfg: PasswordConfig) -> Password {
            Password {
                length: cfg.0,
                with_upper_case: cfg.1,
                with_spec_char: cfg.2,
                with_digit: cfg.3,
            }
        }

        pub fn get_config(&self) -> PasswordConfig {
            (
                self.length,
                self.with_upper_case,
                self.with_spec_char,
                self.with_digit,
            )
        }

        pub fn update_config(&mut self, cfg: PasswordConfig) -> Result<String, String> {
            // config validation
            if let Err(conf) = config_valid(&cfg) {
                return Err(conf);
            }

            self.length = cfg.0;
            self.with_upper_case = cfg.1;
            self.with_spec_char = cfg.2;
            self.with_digit = cfg.3;

            Ok(String::from("config updated"))
        }

        pub fn generate(&self) -> String {
            let mut rng = thread_rng();
            let mut result: String = String::from("");

            for _i in 0..self.length {
                if self.with_upper_case && rng.gen() {
                    let n = utils::ContentCategory::UpperCase;
                    let new_s = n.pick_random_at_index();
                    result.push_str(&new_s);

                    continue;
                }

                if self.with_digit && rng.gen() {
                    let n = utils::ContentCategory::Digit;
                    let new_s = n.pick_random_at_index();
                    result.push_str(&new_s);

                    continue;
                }

                if self.with_spec_char && rng.gen() {
                    let n = utils::ContentCategory::SpecialChars;
                    let new_s = n.pick_random_at_index();
                    result.push_str(&new_s);

                    continue;
                }

                let n = utils::ContentCategory::Lowercase;
                let new_s = n.pick_random_at_index();
                result.push_str(&new_s);
            }

            result
        }
    }

    fn config_valid(cfg: &PasswordConfig) -> Result<&PasswordConfig, String> {
        if cfg.0 < 4 {
            return Err(String::from("password length could not be below 4"));
        }

        Ok(cfg)
    }
}

mod utils {
    use rand::{thread_rng, Rng};

    pub enum ContentCategory {
        Lowercase,
        UpperCase,
        Digit,
        SpecialChars,
    }

    impl ContentCategory {
        pub fn pick_random_at_index(&self) -> String {
            let letters: String = String::from("abcdefghijklmnopqrstuvwxyz");
            let digits: String = String::from("0123456789");
            let special_chars: String = String::from("!$%^&*()-_=+{}[]'@#~/?.><,\\|");

            match self {
                ContentCategory::Lowercase => {
                    let max: u8 = letters.len() as u8;
                    let i = get_rand_between(0, max);

                    return pick_in_string(&letters, i);
                }
                ContentCategory::UpperCase => {
                    let max: u8 = letters.len() as u8;
                    let i = get_rand_between(0, max);
                    let uppers = letters.clone().to_uppercase();

                    return pick_in_string(&uppers, i);
                }
                ContentCategory::Digit => {
                    let max: u8 = digits.len() as u8;
                    let i = get_rand_between(0, max);

                    return pick_in_string(&digits, i);
                }
                ContentCategory::SpecialChars => {
                    let max: u8 = special_chars.len() as u8;
                    let i = get_rand_between(0, max);

                    return pick_in_string(&special_chars, i);
                }
            }
        }
    }

    fn get_rand_between(_min: u8, _max: u8) -> u8 {
        let mut rng = thread_rng();

        return rng.gen_range(_min.._max);
    }

    fn pick_in_string(s: &String, i: u8) -> String {
        if let Some(c) = s.chars().nth(i.into()) {
            return c.to_string();
        }

        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use crate::generator;

    #[test]
    fn should_create_pwd() {
        let cfg: generator::PasswordConfig = (8, false, true, false);
        let pwd: generator::Password = generator::Password::new(cfg);

        let test_cfg = pwd.get_config();

        assert_eq!(test_cfg, cfg, "config not equal")
    }

    #[test]
    fn should_update_pwd_config() {
        let default_cfg: generator::PasswordConfig = (8, false, true, false);
        let mut pwd: generator::Password = generator::Password::new(default_cfg);

        let new_cfg: generator::PasswordConfig = (18, true, true, true);

        let res = pwd.update_config(new_cfg);

        match res {
            Ok(_) => {
                assert!(true, "expected config");
            }
            Err(_) => {
                assert!(false, "this config should not throw an error",);
            }
        }
    }

    #[test]
    fn should_not_update_pwd_config() {
        let default_cfg: generator::PasswordConfig = (8, false, true, false);
        let mut pwd: generator::Password = generator::Password::new(default_cfg);

        let new_cfg: generator::PasswordConfig = (2, true, false, true);

        let res = pwd.update_config(new_cfg);

        match res {
            Ok(_) => {
                assert!(false, "this config should throw an error",);
            }
            Err(_) => {
                assert!(true, "expected error");
            }
        }
    }

    #[test]
    fn should_generate_random_pwd() {
        let cfg: generator::PasswordConfig = (8, false, true, false);
        let pwd: generator::Password = generator::Password::new(cfg);

        let gen_pwd1 = pwd.generate();
        let gen_pwd2 = pwd.generate();

        assert_ne!(gen_pwd1, gen_pwd2, "passwords should'nt be equal");
    }
}
