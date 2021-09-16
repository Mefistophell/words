use words::Item;
use std::{io, process, env};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

mod driver;
mod config;

static mut STORE_FILENAME: String = String::new();

fn get_filename() -> String {
    unsafe { STORE_FILENAME.clone() }
}

fn set_filename(name: String) {
    unsafe {
        STORE_FILENAME = name;
    }
}

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let filename = format!("{}.csv", config.language);
    set_filename(filename);

    let data: Vec<Item> = driver::import(&get_filename()).unwrap();
    let shared_data = Arc::new(Mutex::new(data));

    let shared_data_clone = Arc::clone(&shared_data);
    ctrlc::set_handler(move || {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Press Enter to exit");
        let data = &*shared_data_clone.lock().unwrap();
        driver::export(&get_filename(), &data).unwrap();
        process::exit(1);
    }).expect("Error setting Ctrl-C handler");

    if config.action == "add" {
        add(Arc::clone(&shared_data));
    } else if config.action == "get" {
        let count;
        {
            let data = &mut *shared_data.lock().unwrap();
            data.shuffle(&mut thread_rng());
            data.sort_by(|a, b| b.frequency.cmp(&a.frequency));
            count = data.iter().count();
        }
        get(Arc::clone(&shared_data), count);
    } else if config.action == "find" {
        find(Arc::clone(&shared_data))
    }
}

fn find(data: Arc<Mutex<Vec<Item>>>) {
    let data = data.lock().unwrap();
    let line = read_line();
    match data.iter().filter(|item| item.word == line.to_lowercase()).next() {
        Some(item) => {
            println!("The word '{}' exists", item.word);
        }
        None => {
            println!("The word '{}' does not exist", line);
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: unable to read user input");
    let line = input.clone().replace("\n", "");
    line
}

fn get(data: Arc<Mutex<Vec<Item>>>, count: usize) {
    let mut total = count;
    let mut i = 0;
    while i < total {
        {
            let mut data = data.lock().unwrap();
            let item = &mut data[i];
            println!("{}", item.word);
            let line = read_line();

            if line == "/" {
                println!("{} [{}]", item.translation, item.context);
                let line = read_line();
                if line == "/" {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    item.learn();
                } else if line == "del" {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    data.remove(i);
                    println!("The word has been deleted");
                    total -= 1;
                    i -= 1;
                }
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            } else {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("{} — {} [{}]", item.word, item.translation, item.context);
            }
            drop(data);
        }
        thread::sleep(Duration::from_millis(100));
        i += 1;
    }
    let data = data.lock().unwrap();
    driver::export(&get_filename(), &data).unwrap();
}

fn add(data: Arc<Mutex<Vec<Item>>>) {
    println!("word,translation,context\n==============");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let line = input.clone().replace("\n", "");
    let parts: Vec<&str> = line.split(",").collect();
    if parts.len() != 3 {
        eprintln!("Problem parsing arguments");
        process::exit(1);
    }
    let word = parts[0].to_lowercase();
    let item = Item::new(&word, &parts[1].to_lowercase(), parts[2]);

    let mut data = data.try_lock().unwrap();

    data.iter().for_each(|item| {
        if item.word == word {
            eprintln!("Word exists");
            process::exit(1);
        }
    });

    data.push(item);

    driver::export(&get_filename(), &data).unwrap();
    println!("The word `{}` is added to the collection", parts[0]);
}
