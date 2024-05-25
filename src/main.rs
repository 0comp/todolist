use core::panic;
use std::io;
use std::fs;

fn main()   {
    println!("type what you want to add to the list");
    println!();
    println!("type \"stop\" to stop adding stuff");
    println!();
    println!("type \"Show list\" to show the list");
    println!();
    println!("type \"save\" to save the list");
    println!();
    println!("type \"load list\" to load the saved list");
    println!();
    let  list: Vec<String> = get_list();

    
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
        get_item = get_item.replace("\r", "").replace("\n", "");
        
        if get_item == "save" || get_item == "save list" {
            println!("saving your list");
            let _ = save_list(&list);
            continue
        }
        if get_item == "stop" {
            panic!("told_to_stop")
        }
        if get_item == "show list" || get_item == "show" {
            display_list(&list);
            println!("showing your list");
            continue
        }
        if get_item == "load list" || get_item == "load" {
            list = read_file();
            println!("loading your list");
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
    }
}


fn display_list(list: &[String]) {

    for x in list.iter().enumerate() {        
        println!("{} {}", x.0, x.1);
    }
}
fn save_list(list: &Vec<String>) -> io::Result<Vec<String>> {
    fs::write("./todolist", serde_json::to_string(&list).unwrap())?;
    
    return Ok(list.clone())
}

fn read_file()-> Vec<String> {
    let list = String::from_utf8(std::fs::read("./todolist").unwrap()).unwrap();

    serde_json::from_str(&list).unwrap()
}