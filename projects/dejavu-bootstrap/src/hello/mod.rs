mod hello_render;

pub struct HelloTemplate {
    pub name: String,
    pub users: Vec<User>,
}

pub struct User {
    pub name: String,
}
