use std::fs;
use std::fmt::Write;
use std::io::Error;
use words::Item;

const SEPARATOR: &str = ",";

pub fn import(filename: &str) -> Result<Vec<Item>, Error> {
    let contents = fs::read_to_string(filename)?;
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut rows: Vec<Item> = vec![];

    lines.iter().for_each(|item| {
        let params: Vec<&str> = item.split(SEPARATOR).collect();
        if params[0] != "" {
            rows.push(Item {
                word: params[0].to_string(),
                translation: params[1].to_string(),
                context: params[2].to_string(),
                frequency: params[3].parse::<i8>().unwrap()
            });
        }
    });
    Ok(rows)
}

pub fn export(filename: &str, collection: &Vec<Item>) -> Result<(), Error> {
    let mut string = String::new();

    collection.iter().for_each(|item| {
        let res = format!("{}{}{}{}{}{}{}{}\n",
                          item.word, SEPARATOR,
                          item.translation, SEPARATOR,
                          item.context, SEPARATOR,
                          item.frequency, SEPARATOR);
        string.write_str(&res).unwrap();
    });
    fs::write(filename, string)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_import() {
        let filename = "driver_data_test.csv";

        let item1 = Item::new("hello", "hello world", "привет");
        let item2 = Item::new("bye", "bye friend", "пока");
        let collection = vec![item1, item2];

        let data = export(filename, &collection).unwrap();
        assert_eq!(data, ());

        let data = import(filename).unwrap();
        assert_eq!(data[0].word, "hello");
        assert_eq!(data[1].word, "bye");

        fs::remove_file(filename).unwrap();
    }
}
