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

use error::TodoError as TE;
use error::TodoErrorKind as TEK;
use error::ResultExt;
use error::Result;

use libimagstore::store::Entry;

use uuid::Uuid;
use toml::Value;
use toml_query::read::TomlValueReadExt;

pub trait Task {
    fn get_uuid(&self) -> Result<Uuid>;
}

impl Task for Entry {
    fn get_uuid(&self) -> Result<Uuid> {
        match self.get_header().read("todo.uuid") {
            Ok(Some(&Value::String(ref uuid))) => {
                Uuid::parse_str(uuid).chain_err(|| TEK::UuidParserError)
            },
            Ok(Some(_)) => Err(TE::from_kind(TEK::HeaderTypeError)),
            Ok(None)    => Err(TE::from_kind(TEK::HeaderFieldMissing)),
            Err(e)      => Err(e).chain_err(|| TEK::StoreError),
        }
    }
}

