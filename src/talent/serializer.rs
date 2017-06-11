#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalentForm {
    pub name: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Talent {
    pub name: String,
    pub username: String,
}
