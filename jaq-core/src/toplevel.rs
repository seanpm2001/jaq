use crate::{unparse, Filter};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use jaq_parse::{Def, Error, Main};

pub struct Definitions(BTreeMap<(String, usize), Filter>);

impl Definitions {
    pub fn core() -> Self {
        Self(Filter::core().into_iter().collect())
    }

    pub fn add(&mut self, defs: Vec<Def>, errs: &mut Vec<Error>) {
        for def in defs {
            let f = unparse(&self.get(), &def.args, Vec::new(), def.body, errs);
            self.0.insert((def.name, def.args.len()), f);
        }
    }

    pub fn finish(mut self, (defs, body): Main, errs: &mut Vec<Error>) -> Filter {
        self.add(defs, errs);
        unparse(&self.get(), &[], Vec::new(), body, errs)
    }

    fn get(&self) -> impl Fn(&(String, usize)) -> Option<Filter> + '_ {
        |fun| self.0.get(fun).cloned()
    }
}
