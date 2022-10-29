struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotherusername@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername123"),
        ..user1
    };

    let user3 = build_user(
        String::from("athirduser@example.com"),
        String::from("thirdusername123"),
    );

    println!("User 1's username: {}", user1.username);
    println!("User 2's active: {}", user2.active);
    println!("User 3's sign_in_count: {}", user3.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
