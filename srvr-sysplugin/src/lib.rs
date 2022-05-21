use std::fmt::{Display, Formatter};

pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &PluginVersion;

    fn start(&mut self);
    fn stop(&mut self);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PluginVersion(usize, usize, usize);

impl PluginVersion {
    pub const fn new(version_tuple: (usize, usize, usize)) -> Self {
        PluginVersion(version_tuple.0, version_tuple.1, version_tuple.2)
    }
}

impl Display for PluginVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.0, self.1, self.2)
    }
}