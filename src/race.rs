use serde::{Deserialize, Serialize};

use crate::{horse::Horse, title_table::TitleTable};

#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    #[serde(flatten)]
    pub title: TitleTable,
    pub field: Vec<Horse>,
}
