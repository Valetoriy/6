use std::rc::Rc;

use rand::Rng;
use slint::{Model, ModelExt, SharedString, VecModel};

slint::include_modules!();

pub struct App {
    app_window: AppWindow,

    profession_model: Rc<VecModel<Profession>>,
    question_model: Rc<VecModel<Question>>,
    mapping_model: Rc<VecModel<QPMapping>>,

    question_text: SharedString,
}

impl App {
    pub fn new() -> Self {
        let app = App::setup_db();

        app.setup_callbacks();

        app
    }

    pub fn run(&self) {
        self.app_window.run().unwrap();

        self.save_db();
    }

    fn setup_db() -> Self {
        let app_window = AppWindow::new().unwrap();

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path("db/profs.csv")
            .unwrap();
        let profs: Vec<Profession> = rdr.deserialize().flatten().collect();
        let profession_model = Rc::new(slint::VecModel::<Profession>::from(profs));
        app_window
            .global::<ModelAdapter>()
            .set_profession_list(profession_model.clone().into());

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path("db/questions.csv")
            .unwrap();
        let qs: Vec<Question> = rdr.deserialize().flatten().collect();
        let question_model = Rc::new(slint::VecModel::<Question>::from(qs));
        app_window
            .global::<ModelAdapter>()
            .set_question_list(question_model.clone().into());

        let mut rdr = csv::Reader::from_path("db/qp_mapping.csv").unwrap();
        let qpms: Vec<QPMapping> = rdr.deserialize().flatten().collect();
        let mapping_model = Rc::new(slint::VecModel::<QPMapping>::from(qpms));
        app_window
            .global::<ModelAdapter>()
            .set_mapping_list(mapping_model.clone().into());

        let question_text = question_model
            .clone()
            .map(|q| q.question + "?")
            .row_data(0)
            .unwrap_or("Список вопросов пуст".into());
        app_window
            .global::<ModelAdapter>()
            .set_question_text(question_text.clone());

        Self {
            app_window,
            profession_model,
            question_model,
            mapping_model,
            question_text,
        }
    }

    fn setup_callbacks(&self) {
        self.app_window.global::<ModelAdapter>().on_update_mapping({
            let mapping_model = self.mapping_model.clone();
            move |p_index, q_index, value| {
                let (i, mut m) = mapping_model
                    .iter()
                    .enumerate()
                    .find(|(_, m)| m.p_index == p_index && m.q_index == q_index)
                    .unwrap();
                m.value = value;
                mapping_model.set_row_data(i, m);
            }
        });

        // Profession
        self.app_window.global::<ModelAdapter>().on_add_profession({
            let profession_model = self.profession_model.clone();
            let question_model = self.question_model.clone();
            let mapping_model = self.mapping_model.clone();
            move || {
                let max_index = profession_model.iter().map(|p| p.index).max().unwrap_or(-1);
                let new_index = max_index + 1;
                profession_model.push(Profession {
                    index: new_index,
                    prof_name: "Новая профессия".into(),
                    traits: "Личные качества, характерные для новой профессии".into(),
                });

                mapping_model.extend(question_model.iter().map(|q| QPMapping {
                    p_index: new_index,
                    q_index: q.index,
                    value: rand::thread_rng().gen_range(0.0..=1.),
                }));
            }
        });

        self.app_window
            .global::<ModelAdapter>()
            .on_remove_profession({
                let profession_model = self.profession_model.clone();
                let mapping_model = self.mapping_model.clone();
                move |p_index| {
                    let model_index = profession_model
                        .iter()
                        .position(|p| p.index == p_index)
                        .unwrap();
                    profession_model.remove(model_index);

                    let mm: Vec<QPMapping> = mapping_model
                        .iter()
                        .filter(|m| m.p_index != p_index)
                        .collect();
                    mapping_model.set_vec(mm);
                }
            });

        self.app_window
            .global::<ModelAdapter>()
            .on_update_profession({
                let profession_model = self.profession_model.clone();
                move |index, prof_name, traits| {
                    let (i, mut p) = profession_model
                        .iter()
                        .enumerate()
                        .find(|(_, p)| p.index == index)
                        .unwrap();
                    p.prof_name = prof_name;
                    p.traits = traits;
                    profession_model.set_row_data(i, p);
                }
            });

        self.app_window
            .global::<ModelAdapter>()
            .on_index_to_question({
                let question_model = self.question_model.clone();
                move |q_index| {
                    let q = question_model.iter().find(|q| q.index == q_index).unwrap();
                    q.question
                }
            });

        // Question
        self.app_window.global::<ModelAdapter>().on_add_question({
            let profession_model = self.profession_model.clone();
            let question_model = self.question_model.clone();
            let mapping_model = self.mapping_model.clone();
            move || {
                let max_index = question_model.iter().map(|q| q.index).max().unwrap_or(-1);
                let new_index = max_index + 1;
                question_model.push(Question {
                    index: new_index,
                    question: "Новый вопрос".into(),
                });

                mapping_model.extend(profession_model.iter().map(|p| QPMapping {
                    p_index: p.index,
                    q_index: new_index,
                    value: rand::thread_rng().gen_range(0.0..=1.),
                }));
            }
        });

        self.app_window
            .global::<ModelAdapter>()
            .on_remove_question({
                let question_model = self.question_model.clone();
                let mapping_model = self.mapping_model.clone();
                move |q_index| {
                    let model_index = question_model
                        .iter()
                        .position(|q| q.index == q_index)
                        .unwrap();
                    question_model.remove(model_index);

                    let mm: Vec<QPMapping> = mapping_model
                        .iter()
                        .filter(|m| m.q_index != q_index)
                        .collect();
                    mapping_model.set_vec(mm);
                }
            });

        self.app_window
            .global::<ModelAdapter>()
            .on_update_question({
                let question_model = self.question_model.clone();
                move |index, question| {
                    let (i, mut q) = question_model
                        .iter()
                        .enumerate()
                        .find(|(_, q)| q.index == index)
                        .unwrap();
                    q.question = question;
                    question_model.set_row_data(i, q);
                }
            });

        self.app_window
            .global::<ModelAdapter>()
            .on_index_to_profession({
                let profession_model = self.profession_model.clone();
                move |p_index| {
                    let p = profession_model
                        .iter()
                        .find(|p| p.index == p_index)
                        .unwrap();
                    p.prof_name
                }
            });
    }

    fn save_db(&self) {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_path("db/profs.csv")
            .unwrap();
        let profs: Vec<Profession> = self.profession_model.iter().collect();
        for p in profs {
            wtr.serialize(p).unwrap();
        }

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_path("db/questions.csv")
            .unwrap();
        let qs: Vec<Question> = self.question_model.iter().collect();
        for q in qs {
            wtr.serialize(q).unwrap();
        }

        let mut wtr = csv::Writer::from_path("db/qp_mapping.csv").unwrap();
        let qpms: Vec<QPMapping> = self.mapping_model.iter().collect();
        for m in qpms {
            wtr.serialize(m).unwrap();
        }
    }
}
