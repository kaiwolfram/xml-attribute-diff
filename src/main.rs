use clap::{App, Arg, ArgMatches};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashSet;

fn main() -> Result<(), String> {
    let matches = App::new("xml-attribute-diff")
        .version("0.1.0")
        .about("Compare attribute values of two xml files")
        .arg(
            Arg::with_name("file1")
                .help("Original xml file, used as a reference for the comparison")
                .index(1)
                .validator(is_xml_path)
                .required(true),
        )
        .arg(
            Arg::with_name("file2")
                .help("xml file to compare to the original file")
                .index(2)
                .validator(is_xml_path)
                .required(true),
        )
        .get_matches();
    process_cli(&matches)
}

/// Processes command line arguments and executes program
fn process_cli(matches: &ArgMatches) -> Result<(), String> {
    let file1 = matches.value_of("file1").ok_or("File1 is missing")?;
    let file2 = matches.value_of("file2").ok_or("File2 is missing")?;

    let origin = get_attributes_from_xml(file1)?;
    let file = get_attributes_from_xml(file2)?;
    let difference = file.len() as i32 - origin.len() as i32;

    print_diff(difference, file1, file2);
    print_new_attributes(&file.difference(&origin).collect(), file1);
    print_missing_attributes(&origin.difference(&file).collect(), file1);

    Ok(())
}

/// Checks if string ends with ".xml"
fn is_xml_path(path: String) -> Result<(), String> {
    if path.ends_with(".xml") {
        return Ok(());
    }
    Err(format!("[{}] is not an xml file", path))
}

/// Prints the difference between the total number of attributes of two files
fn print_diff(difference: i32, name1: &str, name2: &str) {
    match difference {
        0 => println!("{} has the same amount of attributes as {}", name1, name2),
        1 => println!("{} has one attribute more than {}", name1, name2),
        -1 => println!("{} has one attribute less than {}", name1, name2),
        _ => {
            if difference.is_positive() {
                println!(
                    "{} has {} attributes more than {}",
                    name1, difference, name2
                );
            } else {
                println!(
                    "{} has {} attributes less than {}",
                    name1,
                    difference.abs(),
                    name2
                );
            }
        }
    };
}

/// Prints new attributes
fn print_new_attributes(new_attributes: &Vec<&String>, file_name: &str) {
    match new_attributes.len() {
        0 => println!("{} has no new attributes", file_name),
        1 => println!("{} has one new attribute:", file_name),
        _ => println!("{} has {} new attributes:", file_name, new_attributes.len()),
    }
    print_changes(new_attributes);
}

/// Prints missing attributes
fn print_missing_attributes(missing_attributes: &Vec<&String>, file_name: &str) {
    match missing_attributes.len() {
        0 => println!("{} has no missing attributes", file_name),
        1 => println!("{} has one missing attribute:", file_name),
        _ => println!(
            "{} has {} missing attributes:",
            file_name,
            missing_attributes.len()
        ),
    }
    print_changes(missing_attributes);
}
/// Prints attributes indented
fn print_changes(attributes: &Vec<&String>) {
    attributes.iter().for_each(|attr| println!("\t{}", attr));
}

/// Returns the values of all attributes of a given xml file
fn get_attributes_from_xml(path: &str) -> Result<HashSet<String>, String> {
    let mut reader =
        Reader::from_file(path).map_err(|_| format!("Failed to read file from path [{}]", path))?;
    reader.trim_text(true);

    let mut attributes: HashSet<String> = HashSet::with_capacity(100);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                e.attributes()
                    .map(|a| {
                        String::from_utf8(a.expect("Failed to read attribute").value.to_vec())
                            .expect("Failed to parse attribute")
                    })
                    .for_each(|attr| {
                        attributes.insert(attr);
                        ()
                    });
            }
            Ok(Event::Eof) => break,
            Err(_) => return Err(format!("XML file [{}] is malformatted", path)),
            _ => (),
        }
        buf.clear();
    }

    Ok(attributes)
}
