use std::{fmt::Display, num::NonZeroUsize};

use crate::interface::{BuildAst, Traverse};

#[derive(Debug)]
pub struct Arena(Vec<Ast>);

impl Arena {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(&mut self, itm: Ast) -> Ref {
        let r = Ref(unsafe { NonZeroUsize::new_unchecked(self.0.len() + 1) });
        self.0.push(itm);
        r
    }

    pub fn get(&self, r: Ref) -> &Ast {
        &self.0[r.0.get() - 1]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ref(NonZeroUsize);

#[derive(Debug, Clone, Copy)]
pub enum Ast {
    Str(&'static str),
    Option {
        name: Ref,
        value: Ref,
    },
    Command {
        name: Ref,
        options: Option<Ref>,
        args: Option<Ref>,
        pipeline: Option<Ref>,
    },
    List {
        value: Ref,
        next: Option<Ref>,
    },
}

impl BuildAst for Arena {
    type Head = AstHead;

    type Command = Ref;

    type Option = Ref;

    type OptionList = Ref;

    type ArgList = Ref;

    type Str = Ref;

    fn build_head(self, command: Self::Command) -> Self::Head {
        AstHead(self, command)
    }

    fn build_str(&mut self, s: &'static str) -> Self::Str {
        self.insert(Ast::Str(s))
    }

    fn build_command(
        &mut self,
        name: Self::Str,
        options: Option<Self::OptionList>,
        args: Option<Self::ArgList>,
        pipeline: Option<Self::Command>,
    ) -> Self::Command {
        self.insert(Ast::Command {
            name,
            options,
            args,
            pipeline,
        })
    }

    fn build_option(&mut self, name: Self::Str, value: Self::Str) -> Self::Option {
        self.insert(Ast::Option { name, value })
    }

    fn build_option_list(
        &mut self,
        value: Self::Option,
        next: Option<Self::OptionList>,
    ) -> Self::OptionList {
        self.insert(Ast::List { value, next })
    }

    fn build_arg_list(&mut self, value: Self::Str, next: Option<Self::ArgList>) -> Self::ArgList {
        self.insert(Ast::List { value, next })
    }
}

pub struct AstHead(Arena, Ref);

impl Display for AstHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Ast {
    fn traverse(&self, a: &Arena, s: impl Fn(&str) + Clone) {
        match *self {
            Ast::Str(x) => s(x),
            Ast::Option { name, value } => {
                a.get(name).traverse(a, s.clone());
                a.get(value).traverse(a, s.clone());
            }
            Ast::Command {
                name,
                options,
                args,
                pipeline,
            } => {
                a.get(name).traverse(a, s.clone());
                if let Some(o) = options {
                    a.get(o).traverse(a, s.clone());
                }

                if let Some(x) = args {
                    a.get(x).traverse(a, s.clone());
                }

                if let Some(p) = pipeline {
                    a.get(p).traverse(a, s.clone());
                }
            }
            Ast::List { value, next } => {
                a.get(value).traverse(a, s.clone());
                if let Some(n) = next {
                    a.get(n).traverse(a, s);
                }
            }
        }
    }
}

impl Traverse for AstHead {
    fn traverse(&self, s: impl Fn(&str) + Clone) {
        self.0.get(self.1).traverse(&self.0, s)
    }
}
