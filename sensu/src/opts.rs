//! Generates Shush data structures for `sensu` module from command line flags

use crate::resources::ShushResources;
use crate::expire::Expire;

pub struct SilenceOpts {
    pub resources: Option<ShushResources>,
    pub checks: Option<Vec<String>>,
    pub expire: Expire,
}

pub struct ClearOpts {
    pub resources: Option<ShushResources>,
    pub checks: Option<Vec<String>>,
}

pub struct ListOpts {
    pub sub: Option<String>,
    pub chk: Option<String>,
}

pub enum ShushOpts {
    Silence(SilenceOpts),
    Clear(ClearOpts),
    List(ListOpts),
}
