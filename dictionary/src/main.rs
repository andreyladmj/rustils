use std::cell::{RefCell, RefMut};
use std::vec::Vec;
use eframe::egui;
use rusqlite::{Connection, Result};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 960.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Vocabulary",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<Vocab>::default()
        }),
    )
}

#[derive(Clone, Debug)]
struct Sentence {
    id: u32,
    word_id: u32,
    sentence_eng: String,
    sentence_esp: String,
    sentence_rus: String,
}
impl Sentence {
    fn new() -> Self {
        Self {
            id: 0,
            word_id: 0,
            sentence_eng: "".to_string(),
            sentence_esp: "".to_string(),
            sentence_rus: "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct Word {
    id: u32,
    word_eng: String,
    word_rus: String,
    word_esp: String,
    image_url: String,
    correct: u32,
    showed: u32,
    sentences: Vec<Sentence>
}

impl Word {
    fn New() -> Self {
        Self {
            id: 0,
            word_eng: "".to_string(),
            word_rus: "".to_string(),
            word_esp: "".to_string(),
            image_url: "".to_string(),
            correct: 0,
            showed: 0,
            sentences: vec![]
        }
    }

    // TODO: fetch sentences using sql JOIN
    fn get_sentences(&mut self) -> Result<()> {
        self.sentences.clear();
        let conn = Connection::open("vocab.db3")?;
        let mut stmt = conn.prepare("SELECT id, word_id, sentence_eng, sentence_esp, sentence_rus FROM sentences WHERE word_id = ?1")?;
        let sentences_iter = stmt.query_map(&[&self.id], |row| {
            Ok(Sentence {
                id: row.get(0)?,
                word_id: row.get(1)?,
                sentence_eng: row.get(2)?,
                sentence_esp: row.get(3)?,
                sentence_rus: row.get(4)?,
            })
        })?;

        for sentence in sentences_iter {
            self.sentences.push(sentence.unwrap());
        }
        Ok(())
    }

    fn save_sentence(mut sentence: Sentence) -> Result<()> {
        let conn = Connection::open("vocab.db3")?;

        if sentence.id == 0 {
            conn.execute(
                "INSERT INTO sentences (word_id, sentence_eng, sentence_esp, sentence_rus) values (?1, ?2, ?3, ?4);",
                &[&sentence.word_id.to_string(), &sentence.sentence_eng.to_string(), &sentence.sentence_esp.to_string(), &sentence.sentence_rus.to_string()],
            )?;
        } else {
            conn.execute(
                "UPDATE sentences SET word_id=?1, sentence_eng=?2, sentence_esp=?3, sentence_rus=?4 WHERE id=?5 ;",
                &[&sentence.word_id.to_string(), &sentence.sentence_eng.to_string(), &sentence.sentence_esp.to_string(), &sentence.sentence_rus.to_string(), &sentence.id.to_string()],
            )?;
        }

        let last_id: String = conn.last_insert_rowid().to_string();
        println!("Save sentence: {:?}, last_id: {}", sentence, last_id);
        Ok(())
    }

    fn delete_sentence(&mut self, id: u32) -> Result<()> {
        let conn = Connection::open("vocab.db3")?;
        conn.execute(
            "DELETE FROM sentences WHERE id=?1;",
            &[&id],
        )?;
        Ok(())
    }

    fn append_empty_sentence(&mut self) {
        self.sentences.push(Sentence::new());
    }
}


struct Vocab {
    words: Vec<Word>,
    editing_word: Word,
}

impl Vocab {
    fn save_word(&self, mut word: Word) -> Result<()> {
        if word.word_eng == "" {
            return Ok(());
        }

        let conn = Connection::open("vocab.db3")?;
        let last_id;

        if word.id == 0 {
            conn.execute(
                "INSERT INTO words (word_eng, word_esp, word_rus, image_url, showed, corrected) values (?1, ?2, ?3, ?4, ?5, ?6);",
                &[&word.word_eng, &word.word_esp.to_string(), &word.word_rus.to_string(), &word.image_url.to_string(), &0.to_string(), &0.to_string()],
            )?;
            last_id = conn.last_insert_rowid() as u32;
        } else {
            conn.execute(
                "UPDATE words SET word_eng=?1, word_esp=?2, word_rus=?3, image_url=?4, showed=?5, corrected=?6 WHERE id=?7 ;",
                &[&word.word_eng, &word.word_esp.to_string(), &word.word_rus.to_string(), &word.image_url.to_string(), &0.to_string(), &0.to_string(), &word.id.to_string()],
            )?;
            last_id = word.id;
        }



        for mut sentence in &mut word.sentences {
            sentence.word_id = last_id;
            if let Err(e) = Word::save_sentence(sentence.clone()) {
                println!("Error: {}", e);
            }
        }

        println!("Save word, last_id: {}", last_id);
        Ok(())
    }

    fn get_words(&mut self) -> Result<()> {
        self.words.clear();
        let conn = Connection::open("vocab.db3")?;
        let mut stmt = conn.prepare("SELECT id, word_eng, word_rus, word_esp, image_url, corrected, showed FROM words")?;
        let words_iter = stmt.query_map([], |row| {
            Ok(Word {
                id: row.get(0)?,
                word_eng: row.get(1)?,
                word_rus: row.get(2)?,
                word_esp: row.get(3)?,
                image_url: row.get(4)?,
                correct: row.get(5)?,
                showed: row.get(6)?,
                sentences: vec![]
            })
        })?;

        for word in words_iter {
            println!("word: {:?}", word);
            self.words.push(word.unwrap());
        }

        for word in &mut self.words {
            if let Err(e) = word.get_sentences() {
                println!("Error: {}", e);
            }
        }
        Ok(())
    }

    fn delete_word(&mut self, id: u32) -> Result<()> {
        let conn = Connection::open("vocab.db3")?;
        conn.execute(
            "DELETE FROM words WHERE id=?1;",
            &[&id],
        )?;
        conn.execute(
            "DELETE FROM sentences WHERE word_id=?1;",
            &[&id],
        )?;
        Ok(())
    }
}

impl Default for Vocab {
    fn default() -> Self {
        let mut vocab = Self {
            words: Vec::new(),
            editing_word: Word::New(),
        };

        // vocab.selected_option = Some(&mut vocab.empty_word);

        if let Err(e) = vocab.get_words() {
            println!("Error: {}", e);
        }

        vocab
    }
}

impl eframe::App for Vocab {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Words");

            let mut selected = self.editing_word.word_eng.to_string();

            let mut selected_option = self.editing_word.clone();

            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", selected))
                .show_ui(ui, |ui| {
                    for word in &self.words {
                        ui.selectable_value(&mut selected, word.word_eng.clone(), &word.word_eng);
                    }
                }
                );

            if selected != "" {
                if selected_option.word_eng != selected.to_string() {
                    println!("Selected was changed: {}", selected);
                    for word in &self.words {
                        if selected.to_string() == word.word_eng {
                            selected_option = word.clone();
                        }
                    }
                }
            }

            ui.separator();
            let max_width = 1500.0;
            let loop_ = true;
            let mut i = 0;

            while loop_ {
                ui.horizontal(|ui| {
                    ui.set_width(max_width);

                    while i < self.words.len() {
                        if ui
                            .add(egui::SelectableLabel::new(
                                self.words[i].id == selected_option.id,
                                if self.words[i].id == selected_option.id {
                                    selected_option.word_eng.to_string()
                                } else {
                                    self.words[i].word_eng.to_string()
                                }
                            ))
                            .clicked()
                        {
                            selected_option = self.words[i].clone();
                        }

                        i += 1;
                        let pos = ui.next_widget_position();
                        if pos.x > max_width {
                            break;
                        }
                    }
                });

                if i >= self.words.len() {
                    break;
                }
            }


            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Refresh").clicked() {
                    if let Err(e) = self.get_words() {
                        println!("Error: {}", e);
                    }
                }
                if ui.button("Clear").clicked() {
                    selected_option = Word::New();
                }
                if selected_option.id > 0 {
                    if ui.button("Delete").clicked() {
                        if let Err(e) = self.delete_word(selected_option.id) {
                            println!("Error: {}", e);
                        }
                    }
                }
            });

            //
            // ui.horizontal(|ui| {
            //     if ui.button("Clear").clicked() {
            //         self.selected_option = None;
            //     }
            // });
            //
            // let empty_word = RefCell::new(Word::New());
            // let mut editing_word = empty_word.borrow_mut();
            //
            // if !self.selected_option.is_none() {
            //     editing_word = self.selected_option.as_ref().unwrap().borrow_mut(); //self.selected_option.as_ref().unwrap();
            // }
            //
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut selected.to_string())
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut editing_word.correct.clone(), 0..=120).text("age"));
            // if ui.button("Increment correct").clicked() {
            //     editing_word.correct += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", editing_word.word_eng, editing_word.correct));
            //



            ui.horizontal(|ui| {
                let name_label = ui.label("Eng: ");
                ui.text_edit_singleline(&mut selected_option.word_eng).labelled_by(name_label.id);
            });



            ui.horizontal(|ui| {
                let name_label = ui.label("Esp: ");
                ui.text_edit_singleline(&mut selected_option.word_esp).labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Rus: ");
                ui.text_edit_singleline(&mut selected_option.word_rus).labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("image_url: ");
                ui.text_edit_singleline(&mut selected_option.image_url).labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                ui.label(format!("Correct: {}", selected_option.correct));
            });
            ui.horizontal(|ui| {
                ui.label(format!("Showed: {}", selected_option.showed));
            });

            ui.separator();

            for mut sentence in &mut selected_option.sentences {
                ui.horizontal(|ui| {
                    let name_label1 = ui.label("Sentence eng: ");
                    ui.text_edit_singleline(&mut sentence.sentence_eng).labelled_by(name_label1.id);

                    let name_label2 = ui.label("Sentence esp: ");
                    ui.text_edit_singleline(&mut sentence.sentence_esp).labelled_by(name_label2.id);

                    let name_label3 = ui.label("Sentence rus: ");
                    ui.text_edit_singleline(&mut sentence.sentence_rus).labelled_by(name_label3.id);

                });
            }
            if ui.button("Add Sentence").clicked() {
                selected_option.append_empty_sentence();
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Save Word").clicked() {
                    if let Err(e) = self.save_word(selected_option.clone()) {
                        println!("Error: {}", e);
                    }
                }
            });

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));

            self.editing_word = selected_option.clone();
        });
    }
}
