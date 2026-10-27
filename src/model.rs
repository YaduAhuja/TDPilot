use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::utils::camel_to_kebab;

/// RyzenAdj settings stored in a TDPilot profile.
///
/// `None` means the setting was not specified and should not be applied.

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RyzenAdjConfig {
    // Power limits (mW)
    pub stapm_limit: Option<u32>,
    pub fast_limit: Option<u32>,
    pub slow_limit: Option<u32>,
    pub apu_slow_limit: Option<u32>,

    // Time constants (seconds)
    pub stapm_time: Option<f32>,
    pub slow_time: Option<f32>,

    // Current limits (RyzenAdj CLI units)
    pub vrm_current: Option<u32>,
    pub vrmsoc_current: Option<u32>,
    pub vrmmax_current: Option<u32>,
    pub vrmsocmax_current: Option<u32>,
    pub psi0_current: Option<u32>,
    pub psi0soc_current: Option<u32>,

    // Temperature limits (°C)
    pub tctl_temp: Option<u32>,
    pub apu_skin_temp: Option<u32>,
    pub dgpu_skin_temp: Option<u32>,
    pub skin_temp_limit: Option<u32>,

    // Thermal / PROCHOT behavior
    pub prochot_deassertion_ramp: Option<u32>,

    // SoC clock limits
    pub max_socclk_frequency: Option<u32>,
    pub min_socclk_frequency: Option<u32>,

    // Fabric clock limits
    pub max_fclk_frequency: Option<u32>,
    pub min_fclk_frequency: Option<u32>,

    // Video Core Next clock limits
    pub max_vcn: Option<u32>,
    pub min_vcn: Option<u32>,

    // Link clock limits
    pub max_lclk: Option<u32>,
    pub min_lclk: Option<u32>,

    // Integrated GPU clock limits
    pub max_gfxclk: Option<u32>,
    pub min_gfxclk: Option<u32>,

    // Operating modes
    pub power_saving: Option<bool>,
    pub max_performance: Option<bool>,
}

impl RyzenAdjConfig {
    pub fn to_args(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let value = serde_json::to_value(self)?;
        let mut args = Vec::<String>::new();
        if let Some(object) = value.as_object() {
            for (key, value) in object {
                if !value.is_null() {
                    args.push(format!("--{} {}", camel_to_kebab(key), value));
                }
            }
        }
        Ok(args)
    }
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Profile {
    pub name: String,
    pub config: Option<RyzenAdjConfig>
}