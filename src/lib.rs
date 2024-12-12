use plugin_core::plugin_api::Plugin;
use plugin_core::plugin_macro::*;

#[plugin_entry]
#[plugin_exit]
#[derive(Debug)]
pub struct Mod2;
const PLUGIN_NAME: &str = "Mod2";
const VERSION: &str = "0.1.0";
const DESCRIPTION: &str = "LALA";

impl Plugin for Mod2 {
    fn name(&self) -> &str {
        PLUGIN_NAME
    }

    fn version(&self) -> &str {
        VERSION
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn execute(&self, input: &str) -> String {
        format!("Mod2: {}", input)
    }

    fn unload()
    where
        Self: Sized,
    {
        println!("Unloading {} plugin", PLUGIN_NAME);
    }

    fn load() -> Box<dyn Plugin>
    where
        Self: Sized,
    {
        println!("[{}]: Loading...", PLUGIN_NAME);
        Box::new(Mod2)
    }
}
