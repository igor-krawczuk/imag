use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use clap::ArgMatches;

use runtime::Runtime;
use module::Module;

pub struct BM<'a> {
    rt: &'a Runtime<'a>,
}

impl<'a> BM<'a> {

    pub fn new(rt: &'a Runtime<'a>) -> BM<'a> {
        BM {
            rt: rt,
        }
    }

    fn runtime(&self) -> &Runtime {
        &self.rt
    }

}

impl<'a> Module for BM<'a> {

    fn exec(&self, matches: &ArgMatches) -> bool {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "bm"
    }
}

impl<'a> Debug for BM<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "BM");
        Ok(())
    }

}
