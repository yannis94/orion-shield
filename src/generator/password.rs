use crate::utils::{self, ContentCategory};
use rand::{self, thread_rng, Rng};

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

        Ok(String::from("Configuration validated."))
    }

    pub fn generate(&self) -> String {
        let mut rng = thread_rng();
        let mut result: String = String::from("");

        for _i in 0..self.length {
            if self.with_upper_case && rng.gen() {
                let n = ContentCategory::UpperCase;
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
