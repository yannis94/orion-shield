pub mod generator;

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
    use crate::generator::password::{Password, PasswordConfig};

    #[test]
    fn should_create_pwd() {
        let cfg: PasswordConfig = (8, false, true, false);
        let pwd: Password = Password::new(cfg);

        let test_cfg = pwd.get_config();

        assert_eq!(test_cfg, cfg, "config not equal")
    }

    #[test]
    fn should_update_pwd_config() {
        let default_cfg: PasswordConfig = (8, false, true, false);
        let mut pwd: Password = Password::new(default_cfg);

        let new_cfg: PasswordConfig = (18, true, true, true);

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
        let default_cfg: PasswordConfig = (8, false, true, false);
        let mut pwd: Password = Password::new(default_cfg);

        let new_cfg: PasswordConfig = (2, true, false, true);

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
        let cfg: PasswordConfig = (8, false, true, false);
        let pwd: Password = Password::new(cfg);

        let gen_pwd1 = pwd.generate();
        let gen_pwd2 = pwd.generate();

        assert_ne!(gen_pwd1, gen_pwd2, "passwords should'nt be equal");
    }
}
