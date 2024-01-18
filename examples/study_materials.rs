use wanikani_rs::{
    response::{CollectionResponse, ErrorResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
};
use wanikani_rs_model::study_material::StudyMaterial;

#[tokio::main]
async fn main() -> Result<(), ErrorResponse> {
    let client = WanikaniClient::default();
    let params = vec![];

    let study_materials: CollectionResponse<ResourceResponse<StudyMaterial>> =
        client.get_study_materials_filtered(params).await?;
    let d = study_materials.data;
    if d.is_empty() {
        return Ok(());
    }
    let first = d.first().unwrap();
    let id = first.id;
    let study_material = client.get_study_material(id).await?;
    assert_eq!(study_material.data, first.data);
    Ok(())
}
