use jp_landxml::parser::LandXMLParser;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <landxml_file>", args[0]);
        process::exit(1);
    }
    
    let file_path = &args[1];
    
    match LandXMLParser::from_file(file_path) {
        Ok(parser) => {
            match parser.parse() {
                Ok(landxml) => {
                    println!("Successfully parsed LandXML file: {}", file_path);
                    println!("Version: {}", landxml.version);
                    println!("Surfaces: {}", landxml.surfaces.len());
                    println!("Alignments: {}", landxml.alignments.len());
                    
                    // JSON出力
                    if let Ok(json) = serde_json::to_string_pretty(&landxml) {
                        println!("\nJSON representation:");
                        println!("{}", json);
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing LandXML: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    }
}
