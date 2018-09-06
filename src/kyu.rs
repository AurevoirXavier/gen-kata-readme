pub struct Kyu<'a> {
    address: &'a str,
    name: String,
    rank: String,
    project: String,
    description: String,
}

impl<'a> Kyu<'a> {
    pub fn new(address: &str) -> Kyu {
        Kyu {
            address,
            name: String::new(),
            rank: String::new(),
            project: String::new(),
            description: String::new(),
        }
    }

    pub fn war_time(&mut self) {
        self.parse();
        self.write();
    }

    fn parse(&mut self) {
        use regex::Regex;
        use reqwest::get;
        use serde_json::{Value, from_str};
        use select::document::Document;
        use select::predicate::{Predicate, Class, Name};

        let document = Document::from(
            get(self.address)
                .expect("an error occur while downloading")
                .text()
                .unwrap().as_str()
        );

        self.rank = document.find(Class(
            "inner-small-hex"
        ).descendant(Name(
            "span"
        ))).next()
            .unwrap()
            .text();

        let mut data = document.find(Name("script"));
        let v: Value = from_str(
            Regex::new(r"App\.data = (.+)\nApp\.routes")
                .unwrap()
                .captures(
                    &(0..9).fold(None, |_, _| data.next())
                        .expect("failed to parse, since the site updated")
                        .text()
                ).unwrap()
                .get(1)
                .unwrap()
                .as_str()
        ).unwrap();

        self.name = v["challengeName"]
            .as_str()
            .unwrap()
            .to_string();

        self.project = self.name
            .chars()
            .flat_map(|c| match c {
                ' ' => '-'.to_lowercase(),
                _ => c.to_lowercase()
            })
            .collect();

        self.description = format_desc(
            v["description"]
                .as_str()
                .unwrap()
        );
    }

    fn write(&self) {
        let path = format!("{}/{}", self.rank, self.project);

        {
            use std::fs::create_dir_all;
            create_dir_all(&path).expect("failed to create dir");
        }

        {
            use std::env::set_current_dir;
            use std::path::Path;
            set_current_dir(Path::new(&path)).expect("failed to change work dir");
        }

        {
            use std::process::Command;
            Command::new("cargo")
                .args(&["init", "--name", &self.project])
                .output()
                .expect("failed to init project");
        }

        {
            use std::fs::File;
            use std::io::prelude::*;
            let mut f = File::create("README.md").expect("failed to create README");
            f.write(format!("## Detail\n[{}]({})\n", self.name, self.address).as_bytes()).expect("an error occur while writing");
            f.write_all(self.description.as_bytes()).expect("an error occur while writing");
            f.write(b"## Thinking\n").expect("an error occur while writing");
            f.sync_all().expect("an error occur while sync(ing) data");
        }
    }
}

fn format_desc(text: &str) -> String {
    let mut desc = String::new();

    for line in text.lines() {
        if line.starts_with("#") { desc += "\\"; }
        desc += &format!("{}\n", line);
    }

    desc
}
