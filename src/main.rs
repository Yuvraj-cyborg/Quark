use quark::db;
//use std::collections::HashMap;

fn main() {
    //println!("Hello World");
    //let mut kv: HashMap<String, String> = HashMap::new();
    //kv.insert("foo".to_string(), "bar".to_string());
    //println!("{:?}", kv);
    let mut newdb = db::QuarkDB::new();
    match newdb.put("foo","bar"){
        Ok(()) => println!("Insert succeeded"),
        Err(e) => println!("Insert failed: {e}"),
    };
    println!("{:?}", newdb);
}
