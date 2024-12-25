use crate::lexer::Kind;

use super::Parser;

// impl Parser<'_> {
//     pub fn get_opening_fn(&mut self) -> Option<usize> {
//         let mut start = self.cur_token().start;

//         self.bump_any();

//         while !self.opened {
//             if self.eat(Kind::LessThan) {
//                 self.opened = true;
//                 self.closed = false;
//             } else if self.eat(Kind::Eof) {
//                 return None;
//             }
//             self.bump_while_not(Kind::LessThan);
//             start = self.cur_token().start;
//         }
//         Some(start)
//     }
// }
