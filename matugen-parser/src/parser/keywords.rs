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
        let mut start = self.lexer_state.cur_token().start;

        self.lexer_state.bump_any();

        while !self.lexer_state.lexer.opened {
            if self.lexer_state.eat(&self.syntax.keyword_opening[0]) {
                self.lexer_state.lexer.opened = true;
                self.parser_state.closed = false;
            } else if self.lexer_state.eat(&Kind::Eof) {
                return None;
            }
            self.lexer_state
                .bump_while_not(&self.syntax.keyword_opening[1]);
            start = self.lexer_state.cur_token().start;
        }
        Some(start)
    }

    pub fn get_closing(&mut self) -> Result<(), ParseError> {
        println!("STARTING TO CLOSE");
        self.lexer_state.bump_any();
        if self.lexer_state.eat(&self.syntax.keyword_closing[0]) {
            self.parser_state.closed = true;
            self.lexer_state.lexer.opened = false;
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

        while !self.lexer_state.at(&Kind::Eof) {
            if !self.lexer_state.at(&self.syntax.keyword_opening[0]) {
                self.lexer_state
                    .bump_until_not_at(&self.syntax.keyword_opening[0]);
            }

            // We would only get the second bracket at the start without the -1,
            // the opening will ALWAYS have two brackets unlike the closing, which
            // might have an error inside of it (so we dont look ahead for the closing).

            let opening = self.get_opening();

            if opening.is_none() {
                return vec;
            }

            self.parser_state.last_bracket_start = opening.unwrap() - 1;
            let start = self.start_node();

            let mut strings: Vec<TokenValue> = vec![];
            let mut filters: Vec<FilterDefinition> = vec![];

            handle_error(self.collect_strings(&mut strings, &mut filters));

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

        self.lexer_state.bump_while_not(&Kind::String);

        let name = self.lexer_state.cur_token().clone().value;

        let mut filter_args: Vec<TokenValue> = vec![];

        handle_error(self.collect_filter_args(&mut filter_args));

        if self.lexer_state.at(&self.syntax.keyword_closing[0]) {
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
        // self.lexer_state.bump_while_not(&Kind::RBracket);
    }

    fn collect_filter_args(
        &mut self,
        arguments: &mut Vec<TokenValue>,
    ) -> Result<Vec<TokenValue>, ParseError> {
        // THIS SHOULD BE THE FILTER NAME
        self.lexer_state.eat(&Kind::String);

        let mut first_arg = true;

        if !self.lexer_state.eat_ignore_spaces(&Kind::Colon) {
            println!(
                "{}",
                format!("DOESNT HAVE ANY ARGS: {:?}", self.lexer_state.cur_token())
                    .red()
                    .bold()
            );
            self.lexer_state
                .bump_while_not(&self.syntax.keyword_closing[0])
        } else {
            loop {
                match self.lexer_state.cur_kind() {
                    Kind::Space => {
                        self.lexer_state.bump_until_not_at(&Kind::Space);
                    }
                    Kind::String => {
                        if self.parser_state.seen_comma || first_arg {
                            arguments.push(self.lexer_state.cur_token.clone().value);
                            self.lexer_state.bump(&Kind::String);
                            self.parser_state.seen_comma = false;
                            first_arg = false;
                        } else {
                            self.lexer_state
                                .bump_while_not(&self.syntax.keyword_closing[0]);
                            return Err(ParseError::new_from_parser(
                                ParseErrorTypes::FilterArgumentNotSeparated,
                                &self,
                            ));
                        }
                    }
                    Kind::Number => {
                        if self.parser_state.seen_comma || first_arg {
                            arguments.push(self.lexer_state.cur_token.clone().value);
                            self.lexer_state.bump(&Kind::Number);
                            self.parser_state.seen_comma = false;
                            first_arg = false;
                        } else {
                            self.lexer_state
                                .bump_while_not(&self.syntax.keyword_closing[0]);
                            return Err(ParseError::new_from_parser(
                                ParseErrorTypes::FilterArgumentNotSeparated,
                                &self,
                            ));
                        }
                    }
                    Kind::Float => {
                        if self.parser_state.seen_comma || first_arg {
                            arguments.push(self.lexer_state.cur_token.clone().value);
                            self.lexer_state.bump(&Kind::Float);
                            self.parser_state.seen_comma = false;
                            first_arg = false;
                        } else {
                            self.lexer_state
                                .bump_while_not(&self.syntax.keyword_closing[0]);
                            return Err(ParseError::new_from_parser(
                                ParseErrorTypes::FilterArgumentNotSeparated,
                                &self,
                            ));
                        }
                    }
                    kind if *kind == self.syntax.keyword_closing[1] => {
                        break;
                    }
                    Kind::Comma => {
                        if self.parser_state.seen_comma && self.lexer_state.eat(&Kind::Comma) {
                            self.parser_state.seen_comma = false;
                            return Err(ParseError::new_from_parser(
                                ParseErrorTypes::DoubleComma,
                                &self,
                            ));
                        } else {
                            self.parser_state.seen_comma = true;
                            self.lexer_state.bump(&Kind::Comma);
                        }
                    }
                    _ => {
                        self.lexer_state.bump_any();
                        return Err(ParseError::new_from_parser(
                            ParseErrorTypes::UnexpectedFilterArgumentToken,
                            &self,
                        ));
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
        self.lexer_state.bump_while_not(&Kind::String);
        strings.push(self.lexer_state.cur_val().clone());

        self.lexer_state.bump_any();

        while !&self.parser_state.closed && !self.lexer_state.at(&Kind::Eof) {
            match &self.lexer_state.cur_kind() {
                Kind::Dot => {
                    if self.parser_state.seen_dot && self.lexer_state.eat(&Kind::Dot) {
                        self.parser_state.seen_dot = false;
                        return Err(ParseError::new_from_parser(
                            ParseErrorTypes::DoubleDot,
                            &self,
                        ));
                    } else {
                        self.parser_state.seen_dot = true;
                        self.lexer_state.bump(&Kind::Dot);
                    }
                }
                Kind::String => {
                    if self.parser_state.seen_dot {
                        strings.push(self.lexer_state.cur_token.clone().value);
                        self.lexer_state.bump(&Kind::String);
                        self.parser_state.seen_dot = false;
                    } else {
                        self.lexer_state
                            .bump_while_not(&self.syntax.keyword_closing[0]);
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
                kind if **kind == self.syntax.keyword_closing[0] => {
                    return self.get_closing();
                    // if self.lexer_state.eat(Kind::RBracket) {
                    //     self.parser_state.closed = true;
                    //     self.lexer.opened = false;
                    //     println!("closed without filter")
                    // } else {
                    //     println!("fucked the closing");
                    //     break;
                    // }
                }
                Kind::Space => self.lexer_state.bump(&Kind::Space),
                Kind::NewLine => self.lexer_state.bump(&Kind::NewLine),
                Kind::Identifier => self.lexer_state.bump(&Kind::Identifier),
                _ => {
                    println!("{:?}", self.lexer_state.cur_token());
                }
            }
        }
        Ok(())
    }
}
