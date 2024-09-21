use super::password::{Password, PasswordConfig};

// used to handle user actions
pub enum Command {
    GetConfig,
    UpdateConfig(PasswordConfig),
    Generate,
    Quit,
}

pub struct Engine {
    password: Password,
}

impl Engine {
    pub fn new() -> Engine {
        let default_cfg: PasswordConfig = (12, true, true, true);
        let pwd: Password = Password::new(default_cfg);

        Engine { password: pwd }
    }

    pub fn exec(&mut self, cmd: Command) -> Result<String, String> {
        match cmd {
            Command::GetConfig => {
                let cfg = self.password.get_config();
                let has_upper = if cfg.1 { "v" } else { "x" };
                let has_spec_chars = if cfg.2 { "v" } else { "x" };
                let has_digits = if cfg.3 { "v" } else { "x" };

                Ok(format!(
                    "length: {}/256\n[{}] uppercase\n[{}] special chars\n[{}] digits",
                    cfg.0, has_upper, has_spec_chars, has_digits,
                ))
            }
            Command::UpdateConfig(cfg) => self.password.update_config(cfg),
            Command::Generate => Ok(self.password.generate()),
            Command::Quit => Err(String::from("quiting")),
        }
    }
}
