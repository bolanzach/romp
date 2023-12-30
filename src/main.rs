use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Map;
use std::path::Path;

enum ParseResponse {
    Continue,
    Complete,
    Error,
}

enum NodeType {
    Document,
    Resource,
    Enum,
    Service,
    Method,
    Field,
    Attribute,
}

struct RompNodeDocument {
    node_type: NodeType,
    path: String,
    nodes: Map<&str, RompNodeResource>
}

impl RompNodeDocument {
    fn new(file_path: &Path) -> RompNodeDocument {
        RompNodeDocument {
            node_type: NodeType::Document,
            path: file_path.file_name().unwrap().to_str().unwrap().to_string(),
        }
    }

    fn parse_line(&mut self, line: &str) -> ParseResponse {
        ParseResponse::Continue
    }
}

struct RompNodeResource {
    node_type: NodeType,
    resource_name: String,
}

impl RompNodeResource {
    fn new(line: &str) -> RompNodeResource {
        let mut node = RompNodeResource {
            node_type: NodeType::Resource,
            resource_name: String::new(),
        };

        let split: Vec<&str> = line.split("resource").collect();
        if split.len() != 2 {
            return panic!("invalid resource line")
        }

        let mut chars = split.get(1).unwrap().chars();
        let mut resource_name = String::new();

        loop {
            let c = chars.next().unwrap();
            if c == ' ' {
                if resource_name.len() == 0 {
                    continue;
                }
                break;
            }

            if c.is_alphanumeric() {
                resource_name.push(c);
            }
        }

        node.resource_name = resource_name;

        return node
    }

    fn parse_line(&mut self, line: &str) -> ParseResponse {
        // if self.resource_name.len() > 0 {
        //     return ParseResponse::Error
        // }

        // let split: Vec<&str> = line.split("resource").collect();
        // if split.len() != 2 {
        //     return ParseResponse::Error
        // }
        //
        // let mut chars = line.chars();
        // let mut resource_name = String::new();
        //
        // loop {
        //     let c = chars.next().unwrap();
        //     if c == ' ' {
        //         break;
        //     }
        //     resource_name.push(c);
        // }
        //
        // self.resource_name = resource_name;

        ParseResponse::Continue
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let path = Path::new("example.rp");
    parse_file(path)
}

fn parse_file(file_path: &Path) {
    let file = match File::open(file_path) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };

    let romp_doc = RompNodeDocument::new(file_path);

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut node: RompNodeResource;

    for line in lines {
        let line = line.unwrap();
        println!("{}", line);

        if is_resource(&line) {
            node = RompNodeResource::new(&line);
            node.parse_line(&line);
        }

        //dbg!(idk);
        // line.chars().co
    }


    println!("----- done")

}

fn is_resource(x: &str) -> bool {
    x.contains("resource ")
}

fn trim_whitespace(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}