use clap::{App, Arg, ArgMatches};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashSet;

fn main() -> Result<(), String> {
    let matches = App::new("xml-attribute-diff")
        .version("0.1.0")
        .about("Compare unique attribute values of two xml files")
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

    print_diff(difference);
    print_new_attributes(&file.difference(&origin).collect());
    print_missing_attributes(&origin.difference(&file).collect());

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
fn print_diff(difference: i32) {
    match difference {
        0 => println!("file2 has the same amount of attribute values as file1"),
        1 => println!("file2 has one attribute value more than file1"),
        -1 => println!("file2 has one attribute value less than file1"),
        _ => {
            if difference.is_positive() {
                println!("file2 has {} attribute values more than file1", difference);
            } else {
                println!(
                    "file2 has {} attribute values less than file1",
                    difference.abs(),
                );
            }
        }
    };
}

/// Prints new attributes
fn print_new_attributes(new_attributes: &Vec<&String>) {
    match new_attributes.len() {
        0 => println!("file2 has no new attribute values"),
        1 => println!("file2 has one new attribute value:"),
        _ => println!("file2 has {} new attribute values:", new_attributes.len()),
    }
    print_changes(new_attributes);
}

/// Prints missing attributes
fn print_missing_attributes(missing_attributes: &Vec<&String>) {
    match missing_attributes.len() {
        0 => println!("file2 has no missing attribute values"),
        1 => println!("file2 has one missing attribute value:"),
        _ => println!(
            "file2 has {} missing attribute values:",
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
