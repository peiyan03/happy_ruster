fn main() {
    // Tuples
    let _rect = (10, 20);

    // Structs
    struct Book {
        title: String,
        author: String,
        pages: u32,
        price: f32,
        avilable: bool,
    }

    struct User{
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // instance of User
    let mut _user1 = User {
        username: "name1".to_string(),
        email: "no@gmail.com".to_string(),
        sign_in_count: 1 ,
        active: true,
    };

    _user1.email = "2@m.com".to_string();
    println!("User email: {}", _user1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            sign_in_count: 1,
            active: true,
        }
    }

    let _user2 = build_user("hey@m.com".to_string(), "name2".to_string());
    println!("User2 email: {}", _user2.email);

    // Create omstamce from another instance
    let _user3 = User {
        email: "user3@mppmpm".to_string(),
        .._user2
    }; // remaining fields from user2, except email
    println!("User3 username: {} {}", _user3.username, _user3.username);

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-like Structs
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

}
