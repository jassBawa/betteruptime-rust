use std::sync::{Arc, Mutex};

use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::{CreateWebsiteOutput, GetWebsiteOutput};
use poem::{
    handler,
    web::{Data, Json, Path},
};
use store::store::Store;

#[handler]
pub fn get_website(
    Path(id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();
    Json(GetWebsiteOutput { url: website.url })
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s
        .create_website(
            String::from("beb436a2-1116-48a3-9521-46f39e0296e9"),
            data.url,
        )
        .unwrap();

    let response = CreateWebsiteOutput { id: website.id };

    Json(response)
}
