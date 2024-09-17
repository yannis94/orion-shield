use orion_shield::generator::{
    engine::{Command, Engine},
    terminal_interaction,
};

fn main() {
    terminal_interaction::terminal_intercation::print_menu();
    let mut engine = Engine::new();
    let res = engine.exec(Command::GetConfig);

    match res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("error: {}", e),
    }

    println!("");
    for _i in 0..5 {
        let pwd = engine.exec(Command::Generate);

        match pwd {
            Ok(res) => println!("Password: {}", res),
            Err(e) => println!("fail to generate password: {}", e),
        }
    }
}
