struct User {
    username: String,
    age: u8,
    email: String,
    active: bool
}

struct Color(u32, u32, u32);

fn main() {
    let user1 = User {
        username: String::from("bobo"),
        age: 20,
        email: String::from("bo@bo.com"),
        active: true
    };

    let mut user2 = User {
        username: String::from("notbo"),
        age: 40,
        email: String::from("not@bo.com"),
        active: false
    };

    user2.age = 30;

    let user3 = build_user(String::from("bobob"), String::from("sdgfsdf"));

    let user4 = User {
        active: false,
        ..user1
    };

    let color = Color(1,2,3);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        age: 10
    }
}
