use std::fmt::{self, Display, Formatter};

use crate::interface::{BuildAst, Traverse};

#[derive(Debug)]
pub struct BoxBuilder;

#[derive(Debug)]
pub enum Ast {
    Str(&'static str),
    Option {
        name: Box<Ast>,
        value: Box<Ast>,
    },
    Command {
        name: Box<Ast>,
        options: Option<Box<Ast>>,
        args: Option<Box<Ast>>,
        pipeline: Option<Box<Ast>>,
    },
    List {
        value: Box<Ast>,
        next: Option<Box<Ast>>,
    },
}

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Traverse for Ast {
    fn traverse(&self, s: impl Fn(&str) + Clone) {
        match self {
            Ast::Command {
                name,
                options,
                args,
                pipeline,
            } => {
                name.traverse(s.clone());
                if let Some(o) = options {
                    o.traverse(s.clone());
                }
                if let Some(a) = args {
                    a.traverse(s.clone());
                }
                if let Some(p) = pipeline {
                    p.traverse(s.clone());
                }
            }
            Ast::Str(x) => s(x),
            Ast::Option { name, value } => {
                name.traverse(s.clone());
                value.traverse(s);
            }
            Ast::List { value, next } => {
                value.traverse(s.clone());
                if let Some(n) = next {
                    n.traverse(s)
                }
            }
        }
    }
}

impl BuildAst for BoxBuilder {
    type Head = Ast;
    type Command = Box<Ast>;
    type Option = Box<Ast>;
    type OptionList = Box<Ast>;
    type ArgList = Box<Ast>;
    type Str = Box<Ast>;

    fn build_head(self, command: Self::Command) -> Self::Head {
        *command
    }

    fn build_str(&mut self, s: &'static str) -> Self::Str {
        Box::new(Ast::Str(s))
    }

    fn build_command(
        &mut self,
        name: Self::Str,
        options: Option<Self::OptionList>,
        args: Option<Self::ArgList>,
        pipeline: Option<Self::Command>,
    ) -> Self::Command {
        Box::new(Ast::Command {
            name,
            options,
            args,
            pipeline,
        })
    }

    fn build_option(&mut self, name: Self::Str, value: Self::Str) -> Self::Option {
        Box::new(Ast::Option { name, value })
    }

    fn build_option_list(
        &mut self,
        value: Self::Option,
        next: Option<Self::OptionList>,
    ) -> Self::OptionList {
        Box::new(Ast::List { value, next })
    }

    fn build_arg_list(&mut self, value: Self::Str, next: Option<Self::ArgList>) -> Self::ArgList {
        Box::new(Ast::List { value, next })
    }
}
