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

    let mw = MainWindow::new().unwrap();
    // let mw_weak = mw.as_weak();
    // mw.on_request_increase_value(move || {
    //     let mw = mw_weak.unwrap();
    //     mw.set_counter(mw.get_counter() + 1);
    // });
    mw.run().unwrap();
}
