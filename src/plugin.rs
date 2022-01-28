use std::path::Path;

use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::plugins::PluginHandler;
use dprint_core::plugins::PluginInfo;

use crate::configuration::{self, Configuration};

pub struct HtmlPluginHandler;

impl HtmlPluginHandler {
    pub const fn new() -> Self {
        HtmlPluginHandler
    }
}

impl PluginHandler<Configuration> for HtmlPluginHandler {
    fn get_plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: String::from(env!("CARGO_PKG_NAME")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            config_key: String::from("html"),
            file_extensions: vec![String::from("html"), String::from("htm")],
            file_names: vec![],
            help_url: String::new(),
            config_schema_url: String::new(),
        }
    }

    fn get_license_text(&mut self) -> String {
        String::from(include_str!("../LICENSE"))
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        configuration::resolve_config(config, global_config)
    }

    fn format_text(
        &mut self,
        _file_path: &Path,
        file_text: &str,
        config: &Configuration,
        mut _format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String>,
    ) -> Result<String> {
        hast::format(&file_text, &hast::Configuration {
            indent_width: config.indent_width,
            line_width: config.line_width,
        })
    }
}
