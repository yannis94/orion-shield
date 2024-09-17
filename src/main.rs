use orion_shield::generator::{
    engine::{Command, Engine},
    password::PasswordConfig,
    terminal_interaction,
};

fn main() {
    terminal_interaction::terminal_intercation::print_menu();
    let mut engine = Engine::new();
    let res = engine.exec(Command::GetConfig);

    match res {
        Ok(s) => print!("{}", s),
        Err(e) => print!("error: {}", e),
    }

    let c: PasswordConfig = (17, false, true, false);
    let p = engine.exec(Command::UpdateConfig(c));

    match p {
        Ok(s) => print!("{}", s),
        Err(e) => print!("error: {}", e),
    }

    print!("\n\n");
    let pwd = engine.exec(Command::Generate);

    match pwd {
        Ok(s) => print!("{}", s),
        Err(e) => print!("error: {}", e),
    }

    print!("\n\n");
    let res = engine.exec(Command::GetConfig);

    match res {
        Ok(s) => print!("{}", s),
        Err(e) => print!("error: {}", e),
    }

    print!("\n\n");
    let pwd = engine.exec(Command::Generate);

    match pwd {
        Ok(s) => print!("{}", s),
        Err(e) => print!("error: {}", e),
    }
}
