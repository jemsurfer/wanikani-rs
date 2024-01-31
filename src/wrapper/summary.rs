use crate::{
    get,
    response::{Error, SummaryResponse, WanikaniError},
    wanikani_client::WanikaniClient,
};

impl WanikaniClient {
    get!(get_summary, "summary", SummaryResponse);
}
