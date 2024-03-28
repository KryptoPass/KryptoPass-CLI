mod kplib;
use openssl::init;

// use kplib::generators::password::PasswordEngine;
use kplib::generators::rules::PasswordRules;

fn main() {
    init();
    let rules_file = String::from("F:\\kryptopass-cli\\Rules.toml");
    let rules = PasswordRules::from_file(rules_file);

    // let password_engine = PasswordEngine::from_rules(rules);
    // let my_password = password_engine.generate();
    // println!("My Password is: {}", my_password);
}
