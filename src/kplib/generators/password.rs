// use crate::kplib::generators::rules::{Length, PasswordRules};
// use crate::kplib::random::Random;

// pub struct PasswordEngine {
//     rules: PasswordRules,
// }

// impl PasswordEngine {
//     pub fn from_rules(rules: PasswordRules) -> Self {
//         PasswordEngine { rules }
//     }

//     fn get_length(&self) -> i32 {
//         match &self.rules.length {
//             Length::Number(n) => *n,
//             Length::Range { min, max } => match Random::rand_range(*min, *max) {
//                 Some(num) => num,
//                 None => 10,
//             },
//         }
//     }

//     pub fn generate(&self) -> String {
//         let length = self.get_length();
//         println!("Num: {}", length);
//         String::new()
//     }
// }
