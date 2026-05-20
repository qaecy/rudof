use napi::Error;
use napi_derive::napi;
use rudof_lib::RudofConfig;

fn cnv_config_err(e: impl std::error::Error) -> Error {
    Error::from_reason(e.to_string())
}

/// Configuration for a Rudof instance.
///
/// Can be created with defaults or loaded from a TOML config file.
#[napi(js_name = "RudofConfig")]
pub struct JsRudofConfig {
    pub(crate) inner: RudofConfig,
}

#[napi]
impl JsRudofConfig {
    /// Creates a new `RudofConfig` with default settings.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: RudofConfig::new(),
        }
    }

    /// Loads a `RudofConfig` from a TOML file path.
    ///
    /// @param path - Absolute or relative path to the config file.
    #[napi(factory)]
    pub fn from_path(path: String) -> napi::Result<Self> {
        let config = RudofConfig::from_path(std::path::Path::new(&path)).map_err(cnv_config_err)?;
        Ok(Self { inner: config })
    }
}
