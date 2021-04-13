#[path = "./ansi.rs"]
mod ansi;
#[path = "./requests.rs"]
mod request;

use chrono;
use serde_json::json;
use std::env;
use std::io::Read;
use std::io::{BufWriter, Write};
use std::path::Path;
#[derive(serde::Deserialize, serde::Serialize)]
struct CachedFile {
    date: String,
}
pub fn cache() {
    let path = format!(
        r#"{}\{}\{}"#,
        env::var("USERPROFILE").unwrap(),
        r#"AppData\Roaming"#,
        r#"Joule\data\db.json"#,
    );
    let json_val = json!({ "date":  chrono::offset::Local::now().format("%Y %b %d %H:%M:%S%.3f %z").to_string() });
    if Path::new(&path).exists() {
        let mut s = String::new();
        let _contents = std::fs::File::open(&path)
            .expect("Something went wrong reading the file")
            .read_to_string(&mut s);
        let json: CachedFile = serde_json::from_str(&s).unwrap();
        let today_date = chrono::offset::Local::now();
        let last_date = chrono::DateTime::parse_from_str(&json.date, "%Y %b %d %H:%M:%S%.3f %z");
        let num_days = today_date
            .signed_duration_since(last_date.unwrap())
            .num_days();

        if num_days >= 7 {
            println!(
                "You have an {} version of packagelists. Updating..., this might take a second",
                ansi::blue("outdated")
            );
            let db_list = &format!(
                r#"{}\{}\{}"#,
                env::var("USERPROFILE").unwrap(),
                r#"AppData\Roaming"#,
                r#"Joule\data\database-list.json"#
            );
            let d = request::download_file("https://raw.githubusercontent.com/joule-package-manager/joule-packages/main/packages.json");
            let mut database_list_file = std::fs::File::create(&db_list)
                .expect(&format!("{}", ansi::red("Failed to create file db.json"),));
            let _ = database_list_file.write(d.as_bytes());
            let mut file = std::fs::File::create(&path)
                .expect(&format!("{}", ansi::red("Failed to create file db.json"),));
            let _ = file.write_all(json_val.to_string().as_bytes());
        }
    } else {
        let db_list = &format!(
            r#"{}\{}\{}"#,
            env::var("USERPROFILE").unwrap(),
            r#"AppData\Roaming"#,
            r#"Joule\data\dblist.json"#
        );
        //? Create DB.json
        let file = std::fs::File::create(&path)
            .expect(&format!("{}", ansi::red("Failed to create file db.json"),));
        let mut f_write = BufWriter::new(file);

        //? Write json_val to DB.json
        let _ = f_write.write_all(json_val.to_string().as_bytes());

        //? Get the DB-list.json in the remote package repo
        let d = request::download_file("https://raw.githubusercontent.com/joule-package-manager/joule-packages/main/packages.json");
        //? Create DB-list.json
        let database_list_file = std::fs::File::create(&db_list)
            .expect(&format!("{}", ansi::red("Failed to create file db.json"),));
        let mut f_write = BufWriter::new(database_list_file);

        //? Write to db-list.json
        match f_write.write(d.as_bytes()) {
            Ok(_) => print!(""),
            Err(err) => println!("{}", ansi::red(&err.to_string())),
        };
    }
}
