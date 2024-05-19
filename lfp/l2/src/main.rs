slint::include_modules!();

#[derive(Debug, serde::Deserialize)]
struct Profession {
    index: usize,
    prof_name: String,
    traits: String,
}

// autocmd BufEnter *.slint :setlocal filetype=slint
#[derive(Debug, serde::Deserialize)]
struct Question {
    index: usize,
    question: String,
}

fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("db/profs.csv")
        .unwrap();
    let profs: Vec<Profession> = rdr.deserialize().flatten().collect();
    for p in profs {
        println!("{p:?}");
    }

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("db/questions.csv")
        .unwrap();
    let qs: Vec<Question> = rdr.deserialize().flatten().collect();
    for q in qs {
        println!("{q:?}");
    }

    let mm = MainMenu::new().unwrap();
    // let mm_weak = mm.as_weak();
    mm.on_consult(move || {
        println!("consult");
    });
    mm.on_profession_list(move || {
        println!("p_list");
    });
    mm.on_question_list(move || {
        println!("q_list");
    });
    mm.run().unwrap();
}
