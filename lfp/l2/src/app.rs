use std::rc::Rc;

use rand::Rng;
use slint::{format, Model, VecModel};

slint::include_modules!();

pub struct App {
    app_window: AppWindow,

    profession_model: Rc<VecModel<Profession>>,
    question_model: Rc<VecModel<Question>>,
    mapping_model: Rc<VecModel<QPMapping>>,

    prof_coef_model: Rc<VecModel<PCoef>>,
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
        let profession_model = Rc::new(slint::VecModel::from(profs));
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
        let mapping_model = Rc::new(slint::VecModel::from(qpms));
        app_window
            .global::<ModelAdapter>()
            .set_mapping_list(mapping_model.clone().into());

        let mut current_question = question_model.clone().row_data(0).unwrap_or(Question {
            index: -1,
            question: "".into(),
        });
        current_question.question += "?";
        app_window
            .global::<ModelAdapter>()
            .set_current_question(current_question.clone());

        let coefs: Vec<PCoef> = profession_model
            .iter()
            .map(|p| PCoef {
                index: p.index,
                coef: 1.,
            })
            .collect();
        let prof_coef_model = Rc::new(slint::VecModel::from(coefs));
        app_window
            .global::<ModelAdapter>()
            .set_profession_coefs(prof_coef_model.clone().into());

        Self {
            app_window,
            profession_model,
            question_model,
            mapping_model,
            prof_coef_model,
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

        // Consult
        let app_weak = self.app_window.as_weak();
        self.app_window
            .global::<ModelAdapter>()
            .on_restart_consult({
                let profession_model = self.profession_model.clone();
                let question_model = self.question_model.clone();
                let prof_coef_model = self.prof_coef_model.clone();
                let app_window = app_weak.unwrap();
                move || {
                    // Сбросить коэффициенты
                    let coefs: Vec<PCoef> = profession_model
                        .iter()
                        .map(|p| PCoef {
                            index: p.index,
                            coef: 1.,
                        })
                        .collect();
                    prof_coef_model.set_vec(coefs);

                    // Заново установить первый вопрос
                    let mut current_question = question_model.row_data(0).unwrap_or(Question {
                        index: -1,
                        question: "".into(),
                    });
                    current_question.question += "?";
                    app_window
                        .global::<ModelAdapter>()
                        .set_current_question(current_question);
                }
            });

        let app_weak = self.app_window.as_weak();
        self.app_window
            .global::<ModelAdapter>()
            .on_question_answer({
                let prof_coef_model = self.prof_coef_model.clone();
                let mapping_model = self.mapping_model.clone();
                let question_model = self.question_model.clone();
                let profession_model = self.profession_model.clone();
                let app_window = app_weak.unwrap();
                move |answer| {
                    let current_question =
                        app_window.global::<ModelAdapter>().get_current_question();

                    // Обновление коэффициентов профессий
                    for i in 0..prof_coef_model.row_count() {
                        let mut current_prof_coef = prof_coef_model.row_data(i).unwrap();
                        let mut coef_value = mapping_model
                            .iter()
                            .find(|m| {
                                m.p_index == current_prof_coef.index
                                    && m.q_index == current_question.index
                            })
                            .unwrap()
                            .value;

                        if !answer {
                            coef_value = 1. - coef_value;
                        }

                        current_prof_coef.coef *= coef_value;
                        prof_coef_model.set_row_data(i, current_prof_coef);
                    }

                    // Проверка коэффициентов
                    let sum = prof_coef_model.iter().map(|pc| pc.coef).sum::<f32>();
                    if sum == 0. {
                        app_window
                            .global::<ModelAdapter>()
                            .set_consult_status(ConsultStatus::NoResults);
                        return;
                    }

                    // Поиск следующего вопроса
                    let mut next_question_index = question_model
                        .iter()
                        .position(|q| q.index == current_question.index)
                        .unwrap()
                        + 1;
                    while next_question_index < question_model.row_count() {
                        let next_question = question_model.row_data(next_question_index).unwrap();

                        let mut coef_sum = 0.;
                        for m in mapping_model
                            .iter()
                            .filter(|m| m.q_index == next_question.index)
                        {
                            coef_sum += m.value
                                * prof_coef_model
                                    .iter()
                                    .find(|pc| pc.index == m.p_index)
                                    .unwrap()
                                    .coef;
                        }

                        if coef_sum != 0. {
                            break;
                        }
                        next_question_index += 1;
                    }

                    // Установление нового вопроса или завершение консультации
                    match question_model.row_data(next_question_index) {
                        Some(mut next_question) => {
                            app_window.global::<ModelAdapter>().set_current_question({
                                next_question.question = format!(
                                    "{}/{}\n{}?",
                                    next_question_index + 1,
                                    question_model.row_count(),
                                    next_question.question
                                );
                                next_question
                            });
                        }
                        None => {
                            let mut top5_profs: Vec<PCoef> =
                                prof_coef_model.iter().filter(|pc| pc.coef != 0.).collect();
                            top5_profs.sort_by(|a, b| b.coef.partial_cmp(&a.coef).unwrap());

                            let mut result_string =
                                String::from("Наиболее подходящие вам профессии:\n");
                            for (i, pc) in top5_profs.iter().enumerate().take(5) {
                                let prof_name = profession_model
                                    .iter()
                                    .find(|p| p.index == pc.index)
                                    .unwrap()
                                    .prof_name;
                                result_string.push_str(&format!("{}. {}\n", i + 1, prof_name));
                            }

                            result_string.push_str("\nВаши личные качества:\n");
                            let traits: String = profession_model
                                .iter()
                                .find(|p| p.index == top5_profs[0].index)
                                .unwrap()
                                .traits
                                .into();
                            let traits: Vec<_> = traits
                                .split('.')
                                .map(|line| std::format!("● {}\n", line.trim()))
                                .collect();
                            result_string.extend(traits);

                            app_window
                                .global::<ModelAdapter>()
                                .set_result_text(result_string.into());
                            app_window
                                .global::<ModelAdapter>()
                                .set_consult_status(ConsultStatus::ShowingResults);
                        }
                    }
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
