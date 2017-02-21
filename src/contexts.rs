#[derive(Serialize)]
pub struct User {
    pub user: String,
    pub movies: Vec<String>,
}

#[derive(Serialize)]
pub struct NotFound {
    pub uri: String,
}

#[derive(Serialize)]
pub struct Login {
    pub message: String,
}

#[derive(Serialize)]
pub struct Empty {
    /* nothing here */
}
