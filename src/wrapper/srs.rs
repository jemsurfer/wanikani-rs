use crate::{
    client::Client,
    get,
    response::{CollectionResponse, IdResponse},
};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use wanisabi_model::spaced_repetition_system::SpacedRepetitionSystem;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SpacedRepetitionSystemFilter {
    Ids(Vec<i64>),
    UpdatedAfter(DateTime<Utc>),
}

impl Client {
    get!(
        get_spaced_repetition_systems_filtered,
        "spaced_repetition_systems",
        SpacedRepetitionSystemFilter,
        CollectionResponse<SpacedRepetitionSystem>
    );
    get!(
        get_spaced_repetition_systems,
        "spaced_repetition_systems",
        CollectionResponse<SpacedRepetitionSystem>
    );
    get!(
        get_spaced_repetition_system,
        "spaced_repetition_systems/{id}",
        IdResponse<SpacedRepetitionSystem>,
        id: i64
    );
}
