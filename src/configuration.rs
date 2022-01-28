use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::configuration::DEFAULT_GLOBAL_CONFIGURATION;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_width: u32,
    pub indent_width: u8,
    /*
    pub sort_attributes: bool,
    pub attributes_ordering: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    */
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            line_width: DEFAULT_GLOBAL_CONFIGURATION.line_width,
            indent_width: 2,
            /*
            sort_attributes: false,
            attributes_ordering: {
                let order: &[(&str, &[&str])] = &[
                    // Default order
                    ("", &["id", "class"]),
                    ("meta", &["name", "content"]),
                    ("link", &["rel", "href"]),
                    ("script", &["type", "src"]),
                ];

                order
                    .into_iter()
                    .map(|(tag, order)| {
                        (
                            Cow::from(*tag),
                            order
                                .into_iter()
                                .map(|&attr| Cow::from(attr))
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<HashMap<_, _>>()
            },
            */
        }
    }
}

pub fn resolve_config(
    mut config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut diagnostics = Vec::new();

    let resolved_config = Configuration {
        line_width: get_value(
            &mut config,
            "lineWidth",
            global_config
                .line_width
                .unwrap_or(DEFAULT_GLOBAL_CONFIGURATION.line_width),
            &mut diagnostics,
        ),
        indent_width: get_value(
            &mut config,
            "indentWidth",
            global_config.indent_width.unwrap_or(2),
            &mut diagnostics,
        ),
        /*
        sort_attributes: get_value(&mut config, "sortAttributes", false, &mut diagnostics),
        ..Configuration::default()
        */
    };

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: resolved_config,
        diagnostics,
    }
}
