use core::panic;
use std::io;
use std::fs;

fn main(){
    println!("type what you want to add to the list");
    println!();
    println!("type \"stop\" to stop adding stuff");
    println!();
    println!("type \"show list\" to show your list");
    println!();
    let list = String::from_utf8(std::fs::read("./todolist").unwrap()).unwrap();
    let mut list: Vec<String>  = serde_json::from_str(&list).unwrap();
    loop {
        let mut get_item: String = String::new();

        io::stdin()
            .read_line(&mut get_item)
            .expect("Failed to read line");
        get_item = get_item.replace("\r", "").replace("\n", "");
        

        if get_item == "stop" {
            panic!("told_to_stop")
        }
        if get_item == "show list" || get_item == "show" {
            display_list(&list);
            println!("showing your list");
            continue
        }
        if let Some(variabel) = get_item.strip_prefix("remove ") {
            let list_length =list.len(); 
            let list_index: usize = variabel.parse().unwrap();
            
            if list_length > list_index{
                println!("removing {}", &variabel);
                list.remove(list_index);
            
            } else  {
                println!("{} does not exist", variabel);
            }
            continue
        }
        
        println!();
        println!("{get_item} has been added to the list");
        println!();
        
        list.push(get_item);
        
        let _ =fs::write("./todolist", serde_json::to_string(&list).unwrap());

    }
}


fn display_list(list: &Vec<String>) {

    for x in list.iter().enumerate() {        
        println!("{} {}", x.0, x.1);
    }
}
