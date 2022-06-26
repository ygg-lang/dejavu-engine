mod basic;

pub struct HelloTemplate {
    name: String,
    item: String,
    users: Vec<User>,
}

pub struct User {
    name: String,
}
