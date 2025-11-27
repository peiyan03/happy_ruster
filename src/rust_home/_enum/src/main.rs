fn main() {
    // enum is similar to struct but it can be one of many different variants
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind) {}

    route(_four);
    route(_six);
    route(IpAddrKind::V6);

     
    struct IpAddr {
        kind: IpAddrKind, // using the enum as a type, it can be either V4 or V6
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("10.1.1.1"),
    };

    print!("IP Address: {} and kind is {}", _home.address, _home.kind as u8);

    /*
    Defining an enum with data associated with each variant
    - we can define the data type in enum 
     */
   
    enum IpAddrWithData {
        V4(String),
        V6(String),
    }

    let _home2 = IpAddrWithData::V4(String::from("10.10.01.1"));
    let _loopback = IpAddrWithData::V6(String::from("::1"));    

    enum NewIp{
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home3 = NewIp::V4(127, 0, 0, 1);
    
    /*
    The match statement checks the variant of the NewIp enum stored in _home3.
        - If _home3 is of the NewIp::V4 variant:  
        - It destructures the V4 variant into its components: a, _, _, _. 
            - a is the first value (e.g., 127 in NewIp::V4(127, 0, 0, 1)).
            -_ is a placeholder for the other values, which are ignored.
        - It prints the first part of the V4 address (127 in this case).
     */
    match _home3 {
        NewIp::V4(a, _, _, _) => println!("First part of V4: {}", a),
        NewIp::V6(_) => println!("This is a V6 address"),
    }
    if let NewIp::V4(a, _, _, _) = _home3 {
        println!("First part of V4: {}", a);
    }

}
