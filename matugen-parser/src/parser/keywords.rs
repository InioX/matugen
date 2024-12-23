use colored::Colorize;

use crate::{
    errors::{
        handle_error, handle_error_panic,
        parse::{ParseError, ParseErrorTypes},
    },
    lexer::{Kind, TokenValue},
    node::{FilterDefinition, KeywordDefinition, Statement},
};

use super::Parser;

impl Parser<'_> {
    fn get_opening(&mut self) -> Option<usize> {
        let mut start = self.cur_token().start;

        self.bump_any();

        while !self.opened {
            if self.eat(Kind::Lbracket) {
                self.opened = true;
                self.closed = false;
            } else if self.eat(Kind::Eof) {
                return None;
            }
            self.bump_while_not(Kind::Lbracket);
            start = self.cur_token().start;
        }
        Some(start)
    }

    pub fn get_closing(&mut self) -> Result<(), ParseError> {
        println!("STARTING TO CLOSE");
        self.bump_any();
        if self.eat(Kind::RBracket) {
            self.closed = true;
            self.opened = false;
            Ok(())
        } else {
            Err(ParseError::new_from_parser(
                ParseErrorTypes::UnclosedBracket,
                &self,
            ))
        }
    }

    pub fn get_keywords(&mut self) -> Vec<Statement> {
        let mut vec: Vec<Statement> = vec![];

        while !self.at(Kind::Eof) {
            if !self.at(Kind::Lbracket) {
                self.bump_until_not_at(Kind::Lbracket);
            }

            // We would only get the second bracket at the start without the -1,
            // the opening will ALWAYS have two brackets unlike the closing, which
            // might have an error inside of it (so we dont look ahead for the closing).
            self.last_bracket_start = self.get_opening().unwrap() - 1;
            let start = self.start_node();

            let mut strings: Vec<TokenValue> = vec![];
            let mut filters: Vec<FilterDefinition> = vec![];

            handle_error_panic(self.collect_strings(&mut strings, &mut filters));

            vec.push(Statement::KeywordDefinition(Box::new(KeywordDefinition {
                node: self.finish_node(start),
                keywords: strings,
                filters: {
                    if filters.len() >= 1 {
                        Some(filters)
                    } else {
                        None
                    }
                },
            })));
        }
        vec
    }

    fn get_filter(&mut self) -> Result<Option<FilterDefinition>, ParseError> {
        let start = self.start_node();

        self.bump_while_not(Kind::String);

        let name = self.cur_token().clone().value;

        let mut filter_args: Vec<TokenValue> = vec![];

        handle_error(self.collect_filter_args(&mut filter_args));

        if self.at(Kind::RBracket) {
            handle_error(self.get_closing());
            return Ok(Some(FilterDefinition {
                node: self.finish_node(start),
                filter_name: name,
                arguments: filter_args,
            }));
        }

        Ok(Some(FilterDefinition {
            node: self.finish_node(start),
            filter_name: name,
            arguments: filter_args,
        }))
        // self.bump_while_not(Kind::RBracket);
    }

    fn collect_filter_args(
        &mut self,
        arguments: &mut Vec<TokenValue>,
    ) -> Result<Vec<TokenValue>, ParseError> {
        // THIS SHOULD BE THE FILTER NAME
        self.eat(Kind::String);

        if !self.eat_ignore_spaces(Kind::Colon) {
            println!(
                "{}",
                format!("DOESNT HAVE ANY ARGS: {:?}", self.cur_token())
                    .red()
                    .bold()
            );
            self.bump_while_not(Kind::RBracket)
        } else {
            // while !self.at(Kind::RBracket) {
            //     match self.cur_kind() {
            //         Kind::String => arguments.push(&self.cur_token.value),
            //         Kind::Number => todo!(),
            //         _ => {}
            //     }
            // }
            loop {
                match self.cur_kind() {
                    Kind::Space => {
                        self.bump_until_not_at(Kind::Space);
                    }
                    Kind::String => {
                        arguments.push(self.cur_token.value.clone());
                        self.bump(Kind::String)
                    }
                    Kind::Number => {
                        arguments.push(self.cur_token.value.clone());
                        self.bump(Kind::Number)
                    }
                    Kind::RBracket => {
                        break;
                    }
                    _ => {
                        return Err(ParseError::new_from_parser(
                            ParseErrorTypes::UnexpectedFilterArgumentToken,
                            &self,
                        ))
                    }
                }
            }
            // return Err(ParseError {
            //     err_type: ParseErrorTypes::MissingFilterColon,
            //     start: self.last_bracket_start,
            //     end: self.prev_token_end,
            //     source: &self.source,
            // });
        }
        println!("arguments: {:?}", arguments);
        Ok(arguments.to_vec())
    }

    // Returns true if filter is used
    fn collect_strings(
        &mut self,
        strings: &mut Vec<TokenValue>,
        filters: &mut Vec<FilterDefinition>,
    ) -> Result<(), ParseError> {
        // Always first string, what comes after we cant know
        self.bump_while_not(Kind::String);
        strings.push(self.cur_val().clone());

        self.bump_any();

        while !&self.closed && !self.at(Kind::Eof) {
            match &self.cur_kind() {
                Kind::Dot => {
                    if self.seen_dot && self.eat(Kind::Dot) {
                        self.seen_dot = false;
                        return Err(ParseError::new_from_parser(
                            ParseErrorTypes::DoubleDot,
                            &self,
                        ));
                    } else {
                        self.seen_dot = true;
                        self.bump(Kind::Dot);
                    }
                }
                Kind::String => {
                    if self.seen_dot {
                        strings.push(self.cur_token.clone().value);
                        self.bump(Kind::String);
                        self.seen_dot = false;
                    } else {
                        self.bump_while_not(Kind::RBracket);
                        return Err(ParseError::new_from_parser(
                            ParseErrorTypes::DoubleString,
                            &self,
                        ));
                    }
                }
                Kind::Bar => {
                    let res = self.get_filter();
                    match res {
                        Ok(v) => {
                            if let Some(def) = v {
                                filters.push(def);
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Kind::RBracket => {
                    return self.get_closing();
                    // if self.eat(Kind::RBracket) {
                    //     self.closed = true;
                    //     self.opened = false;
                    //     println!("closed without filter")
                    // } else {
                    //     println!("fucked the closing");
                    //     break;
                    // }
                }
                Kind::Space => self.bump(Kind::Space),
                Kind::NewLine => self.bump(Kind::NewLine),
                Kind::Identifier => self.bump(Kind::Identifier),
                _ => {
                    println!("{:?}", self.cur_token());
                }
            }
        }
        Ok(())
    }
}
