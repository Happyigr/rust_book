struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let my_color = Color(0, 0, 0);
    let mut user1 = build_user(String::from("popich"), String::from("popich@mail.en"));

    user1.active = false;

    let user2 = User {
        email: String::from("pop@email.com"),
        ..user1
    };
}
