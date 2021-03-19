// Put database functions in this file 
pub fn add_queue(new_queue: String) -> String{ 
    println!("This is the add_queue func in db.rs!");
    println!("from create_queue: {}", new_queue); 
    return "Added queue to database!".to_string()
} 


