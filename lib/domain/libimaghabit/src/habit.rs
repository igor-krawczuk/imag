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

use toml::Value;
use toml_query::insert::TomlValueInsertExt;
use chrono::NaiveDate;

use error::HabitError as HE;
use error::HabitErrorKind as HEK;
use error::*;
use store::HabitStore;
use iter::HabitInstanceIterator;

use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;

const NAIVE_DATE_STRING_FORMAT : &'static str = "%Y-%m-%d";

/// A Habit is a "template" of a habit. A user may define a habit "Eat vegetable".
/// If the user ate a vegetable, she should create a HabitInstance from the Habit with the
/// appropriate date (and optionally a comment) set.
pub trait Habit {

    /// Check whether the instance is a habit by checking its headers for the habit data
    fn is_habit(&self) -> Result<bool>;

    /// Alternative to `HabitStore::create_habit()`
    fn create_instance(&self) -> HabitBuilder {
        HabitBuilder::default()
    }

    fn instances(&self, &Store) -> HabitInstanceIterator;

}

pub struct HabitBuilder {
    name: Option<String>,
    comment: Option<String>,
    date: Option<NaiveDate>,
}

impl HabitBuilder {

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn with_comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn with_date(&mut self, date: NaiveDate) -> &mut Self {
        self.date = Some(date);
        self
    }

    pub fn build<'a>(self, store: &'a Store) -> Result<FileLockEntry<'a>> {
        #[inline]
        fn mkerr(s: &'static str) -> HE {
            HE::from_kind(HEK::HabitBuilderMissing(s))
        }

        let name      = try!(self.name.ok_or_else(|| mkerr("name")));
        let dateobj   = try!(self.date.ok_or_else(|| mkerr("date")));
        let date      = date_to_string(&dateobj);
        let comment   = self.comment.unwrap_or_else(|| String::new());
        let sid       = try!(build_habit_template_sid(&name));
        let mut entry = try!(store.create(sid));

        try!(entry.get_header_mut().insert("habit.template.name", Value::String(name)));
        try!(entry.get_header_mut().insert("habit.template.date", Value::String(date)));
        try!(entry.get_header_mut().insert("habit.template.comment", Value::String(comment)));

        Ok(entry)
    }

}

impl Default for HabitBuilder {
    fn default() -> Self {
        HabitBuilder {
            name: None,
            comment: None,
            date: None,
        }
    }
}

/// Buld a StoreId for a Habit from a date object and a name of a habit
fn build_habit_template_sid(name: &String) -> Result<StoreId> {
    use module_path::ModuleEntryPath;
    ModuleEntryPath::new(format!("template/{}", name).into_storeid().map_err(From::from)
}

fn date_to_string(ndt: &NaiveDate) -> String {
    ndt.format(NAIVE_DATE_STRING_FORMAT).to_string()
}

fn date_from_string(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, NAIVE_DATE_STRING_FORMAT).map_err(From::from)
}

