#[derive(Debug, serde::Deserialize)]
struct Profession {
    prof_name: String,
    traits: String,
}

fn main() {
    let mut rdr = csv::Reader::from_path("profs.csv").unwrap();
    let profs: Vec<Profession> = rdr.deserialize().flatten().collect();
    for p in profs {
        println!("{p:?}");
    }
}
