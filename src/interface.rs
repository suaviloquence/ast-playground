use std::{fmt::Display, iter::Peekable};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Token<'a> {
    Str(&'a str),
    DashDash,
    Pipe,
    Equals,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Str(s) => write!(f, "{s}"),
            Token::DashDash => write!(f, "--"),
            Token::Pipe => write!(f, "|"),
            Token::Equals => write!(f, "="),
        }
    }
}

#[derive(Debug)]
pub struct Parser<S, B> {
    pub scanner: S,
    pub builder: B,
}

pub trait Traverse {
    fn traverse(&self, s: impl Fn(&str) + Clone);
}

pub trait BuildAst {
    type Head: Display + Traverse;
    type Command;
    type Option;
    type OptionList;
    type ArgList;
    type Str;

    fn build_head(self, command: Self::Command) -> Self::Head;
    fn build_str(&mut self, s: &'static str) -> Self::Str;
    fn build_command(
        &mut self,
        name: Self::Str,
        options: Option<Self::OptionList>,
        args: Option<Self::ArgList>,
        next: Option<Self::Command>,
    ) -> Self::Command;

    fn build_option(&mut self, name: Self::Str, value: Self::Str) -> Self::Option;
    fn build_option_list(
        &mut self,
        value: Self::Option,
        next: Option<Self::OptionList>,
    ) -> Self::OptionList;

    fn build_arg_list(&mut self, value: Self::Str, next: Option<Self::ArgList>) -> Self::ArgList;
}

impl<B: BuildAst, I: Iterator<Item = Token<'static>>> Parser<Peekable<I>, B> {
    pub fn parse(mut self) -> B::Head {
        let command = self.parse_command();
        self.builder.build_head(command)
    }

    fn get_str(&mut self) -> B::Str {
        match self.scanner.next() {
            Some(Token::Str(s)) => self.builder.build_str(s),
            x => panic!("Expected Str, got {x:?}"),
        }
    }

    fn parse_command(&mut self) -> B::Command {
        let name = self.get_str();
        let options = self.parse_option_list();
        let args = self.parse_arg_list();
        let pipeline = self.parse_pipeline();

        self.builder.build_command(name, options, args, pipeline)
    }

    fn parse_pipeline(&mut self) -> Option<B::Command> {
        match self.scanner.next() {
            Some(Token::Pipe) => Some(self.parse_command()),
            None => None,
            Some(x) => panic!("Unexpected token {x:?}"),
        }
    }

    fn parse_option_list(&mut self) -> Option<B::OptionList> {
        match self.scanner.peek() {
            Some(Token::DashDash) => {
                let option = self.parse_option();
                let next = self.parse_option_list();
                Some(self.builder.build_option_list(option, next))
            }
            _ => None,
        }
    }

    fn parse_option(&mut self) -> B::Option {
        let Some(Token::DashDash) = self.scanner.next() else {
            panic!("Expected DashDash");
        };
        let name = self.get_str();
        let Some(Token::Equals) = self.scanner.next() else {
            panic!("Expected Equals");
        };
        let value = self.get_str();

        self.builder.build_option(name, value)
    }

    fn parse_arg_list(&mut self) -> Option<B::ArgList> {
        match self.scanner.peek() {
            Some(Token::Str(_)) => {
                let arg = self.get_str();
                let next = self.parse_arg_list();
                Some(self.builder.build_arg_list(arg, next))
            }
            _ => None,
        }
    }
}
