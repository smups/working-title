use srvr_sysplugin::*;

const PLUGIN_NAME: &'static str = "日本語のSample Plugin✴️";
const PLUGIN_VERSION: PluginVersion = PluginVersion::new((0, 0, 1));

#[no_mangle]
pub extern "Rust" fn link() -> Box<dyn Plugin> {Box::new(MyPlugin)}

#[derive(Debug)]
struct MyPlugin;

impl Plugin for MyPlugin {

    fn name(&self) -> &str {PLUGIN_NAME}
    fn version(&self) -> &PluginVersion {&PLUGIN_VERSION}

    fn start(&mut self) {
        println!("Started Plugin {}{}", self.name(), self.version());
    }

    fn stop(&mut self) {
        println!("Stopped Plugin {}{}", self.name(), self.version());
    }
}