pub fn get(key: &Box<Option<String>>){
    println!("{}", key.unwrap());
}

pub fn set(key: &Box<Option<String>>, value: &Box<Option<String>>){
    println!("{} => {}", key.unwrap(), value.unwrap());
}

pub fn delete(key: &Box<Option<String>>){
    println!("{}", key.unwrap());
}

pub fn showall(){
    println!("all");
}