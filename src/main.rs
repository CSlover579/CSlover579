use uuid::Uuid;

fn main() {
    println!("Hello, world!");
    let id = Uuid::new_v4();
    println!("{}",id);
}
