mod hello_render;

pub struct HelloTemplate {
    name: String,
    users: Vec<User>,
}

pub struct User {
    name: String,
}
