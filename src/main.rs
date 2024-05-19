use core::panic;
use std::io;
// use serde;
// use serde_json;




fn main()   {
    println!("type what you want to add to the list");
    println!();
    println!("type \"stop\" to stop adding stuff");
    println!();
    println!("type \"Show list\" to show the list");
    println!();
    println!("type \"save\" to save the list");
    println!();
    let  list = get_list();

    
    dbg!(&list);
    get_list();
}

fn get_list() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    loop {
        let mut get_item = String::new();

        
        io::stdin()
            .read_line(&mut get_item)
            .expect("Failed to read line");
        get_item= get_item.replace("\r", "").replace("\n", "");
        
        if get_item == "save" {
            println!("saving your list");
            return list
        }
        if get_item == "stop" {
            panic!("told_to_stop")
        }
        if get_item == "show list"{
        
            display_list(&list);
          
            continue;
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
    }
}


fn display_list(list: &[String]) {

    for x in list.iter().enumerate() {        
        println!("{} {}", x.0, x.1);
    }
}