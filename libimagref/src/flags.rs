use std::collections::BTreeMap;

use toml::Value;

use error::RefErrorKind as REK;
use result::Result;

#[derive(Default)]
pub struct RefFlags {
    content_hashing:       bool,
    permission_tracking:   bool,
}

impl RefFlags {

    /// Read the RefFlags from a TOML document
    ///
    /// Assumes that the whole TOML tree is passed. So this looks up `ref.flags` to get the flags.
    /// It assumes that this is a Map with Key = <name of the setting> and Value = boolean.
    pub fn read(v: &Value) -> Result<RefFlags> {
        fn get_field(v: &Value, key: &str) -> Result<bool> {
            match v.lookup(key) {
                Some(&Value::Boolean(b)) => Ok(b),
                Some(_) => Err(REK::HeaderTypeError.into()),
                None    => Err(REK::HeaderFieldMissingError.into()),
            }
        }

        Ok(RefFlags {
            content_hashing:     try!(get_field(v, "ref.flags.content_hashing")),
            permission_tracking: try!(get_field(v, "ref.flags.permission_tracking")),
        })
    }

    /// Build a TOML::Value from this RefFlags object.
    ///
    /// Returns a Map which should be set in `ref.flags` in the header.
    pub fn into_toml(self) -> Result<Value> {
        unimplemented!()
    }

    /// Alias for `RefFlags::content_hashing()`
    pub fn is_often_moving(mut self, b: bool) -> RefFlags {
        self.with_content_hashing(b)
    }

    pub fn with_content_hashing(self, b: bool) -> RefFlags {
        unimplemented!()
    }

    pub fn with_permission_tracking(mut self, b: bool) -> RefFlags {
        self.permission_tracking = b;
        self
    }


    pub fn get_content_hashing(&self) -> bool {
        unimplemented!()
    }

    pub fn get_permission_tracking(&self) -> bool {
        unimplemented!()
    }

}

