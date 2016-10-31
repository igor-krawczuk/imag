//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//


use libimagstore::store::Entry;

use filters::filter::Filter;
use rhai::Engine;
use rhai::FnRegister;

pub struct RhaiFilter {
    engine: Engine
}

impl RhaiFilter {

    pub fn new(source: String) -> RhaiFilter {
        let engine = Engine::new();

        engine.regiser_fn("print",      rhai_functions::print);
        engine.regiser_fn("dbg",        rhai_functions::dbg);
        engine.regiser_fn("printerr",   rhai_functions::printerr);

        RhaiFilter {
            source: source,
            engine: engine,
        }
    }

}

impl Filter<Entry> for RhaiFilter {

    fn filter(&self, e: &Entry) -> bool {
        self.0
    }

}

mod rhai_functions {

    pub fn print(s: &mut String) {
        unimplemented!()
    }

    pub fn dbg(s: &mut String) {
        unimplemented!()
    }

    pub fn printerr(s: &mut String) {
        unimplemented!()
    }

}
