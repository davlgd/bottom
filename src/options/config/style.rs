pub mod colours;
pub mod palettes;

use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct StyleConfig {
    pub(crate) color: Option<String>, // TODO: parse enum instead? And how should this react with colour schemes?
}
