use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct JujuConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for JujuConfig<'a> {
    fn default() -> Self {
        JujuConfig {
            format: "via [$symbol$version$model]($style) ",
            symbol: "🔮 ",
            style: "fg:#E95420",
            disabled: true,
        }
    }
}
