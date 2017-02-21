#[derive(FromForm)]
pub struct Login {
    pub name: String,
    pub pass: String,
}

#[derive(FromForm)]
pub struct Register {
    pub name: String,
    pub pass: String,
    // pub email: String,
}
