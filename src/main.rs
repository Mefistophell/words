use words::Item;
use std::{io, process, env};
use rand::thread_rng;
use rand::seq::SliceRandom;

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

    let mut data = driver::import(&get_filename()).unwrap();

    if config.action == "add" {
        add(&mut data);
    } else if config.action == "get" {
        data.shuffle(&mut thread_rng());
        data.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        get(&mut data.clone().iter_mut(), &mut data)
    }
}

fn update_item(item: &mut Item, list: &mut Vec<Item>) {
    //let mut data = driver::import(get_filename()).unwrap();
    let mut data = list.clone();

    let index = data.iter().position(|x| x.word == item.word);
    data.remove(index.unwrap());

    item.learn();

    data.push(Item {
        word: item.word.to_string(),
        context: item.context.to_string(),
        translation: item.translation.to_string(),
        frequency: item.frequency,
    });

    driver::export(&get_filename(), &data).unwrap();
    println!("The word {} updated", item.word);
}

fn remove_item(item: &mut Item, data: &mut Vec<Item>) {
    let index = data.iter().position(|x| x.word == item.word);
    data.remove(index.unwrap());

    driver::export(&get_filename(), &data).unwrap();
    println!("The word {} deleted", item.word);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: unable to read user input");
    let line = input.clone().replace("\n", "");
    line
}

fn get<'a, T: Iterator<Item=&'a mut Item>>(data: &mut T, list: &mut Vec<Item>) {
    println!("=================");
    match data.next() {
        Some(item) => {
            println!("{}", item.word);
            let line = read_line();

            if line == "y" {
                println!("{} [{}]", item.translation, item.context);
                let line = read_line();
                if line == "y" {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    update_item(item, list);
                    get(data, list)
                } else if line == "del" {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    remove_item(item, list);
                    get(data, list)
                }
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                get(data, list)
            } else {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("{} — {} [{}]", item.word, item.translation, item.context);
                get(data, list)
            }
        }
        None => (),
    }
}

fn add(data: &mut Vec<Item>) {
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
    let item = Item::new(&word, parts[1], parts[2]);

    data.iter().for_each(|item| {
        if item.word == word {
            eprintln!("Word exists");
            process::exit(1);
        }
    });

    data.push(item);

    driver::export(&get_filename(), data).unwrap();
    println!("The word `{}` is added to the collection", parts[0]);
}
