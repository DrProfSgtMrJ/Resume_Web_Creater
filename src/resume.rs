use serde_derive::Deserialize;


#[derive(Deserialize)]
pub struct Resume {
    pub title: String,
    pub name: String,
    pub phone: String,
    pub email: String, 
}