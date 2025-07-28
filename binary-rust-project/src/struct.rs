struct User {
active:bool,
username:String,
first_name:String,
last_name:String,
age:i32,

}


fn main {
let user = User {
first_name:String::from("Harkirat"),
last_name:String::from("Singh"),
age:i32::32,
}

println!("{}",user.first_name);
}

// methods, static 