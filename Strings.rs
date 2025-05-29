//Tricky Datatype of Rust ( string)


//First way to initialise a String 
fn main ()
{
let greetings:&str="Hello world";
print!("{}",greeting);

//second way to initialise the string

fn main()
let greetings:String=String::from("Hello world");
printf("{}",greeting);

//Ways to interate a String (2 ways as i know) 
let char1:Option<char>=greeting.char().nth(0);
print!("char1:{}",char1.unwrap());
}


