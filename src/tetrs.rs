use crate::audio;
use crate::board::Board;
use crate::board::BoardSettings;
use crate::ids;
use crate::piece::PieceView;
use crate::save;
use crate::text_art;
use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::event::Event;
use cursive::event::Key;
use cursive::theme::BaseColor;
use cursive::theme::Effect;
use cursive::traits::*;
use cursive::views::DummyView;
use cursive::views::HideableView;
use cursive::views::OnEventView;
use cursive::views::PaddedView;
use cursive::views::TextView;
use cursive::views::{Button, Dialog, LinearLayout};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};

// static atomic state (needed for referncing in cursive callbacks)
static LEVEL: AtomicU8 = AtomicU8::new(1);
static GHOST_PIECE_ON: AtomicBool = AtomicBool::new(true);
static HIGH_SCORE: AtomicU32 = AtomicU32::new(0);
pub fn get_starting_level() -> u8 {
    LEVEL.load(Ordering::Relaxed)
}

pub fn set_level(v: u8) {
    LEVEL.store(v, Ordering::Relaxed)
}

pub fn get_ghost_piece_on() -> bool {
    GHOST_PIECE_ON.load(Ordering::Relaxed)
}

pub fn set_ghost_piece_on(v: bool) {
    GHOST_PIECE_ON.store(v, Ordering::Relaxed)
}
pub fn get_high_score() -> u32 {
    HIGH_SCORE.load(Ordering::Relaxed)
}

pub fn set_high_score(v: u32) {
    HIGH_SCORE.store(v, Ordering::Relaxed);
}

pub fn run() {
    let mut siv = CursiveRunnable::default();
    let mut theme = siv.current_theme().clone();
    theme.palette = cursive::theme::Palette::retro();
    siv.set_theme(theme);

    // fetch high score from disk
    set_high_score(get_high_score_from_disk());
    // init title menu
    show_title_menu(&mut siv);
    // play music on seperate audio thread
    let _ = audio::play(audio::THEME_FAST, true);
    // init cursive
    const FPS: u32 = 60;
    siv.set_fps(FPS);
    siv.run();
    // save highscore on program close
    save_high_score_to_disk(get_high_score());
}

pub fn show_title_menu(s: &mut Cursive) {
    use cursive::theme::Color;
    let title_logo_view = LinearLayout::horizontal()
        .child(TextView::new(text_art::TETRS_T).style(Color::Dark(BaseColor::Red)))
        .child(TextView::new(text_art::TETRS_E).style(Color::Dark(BaseColor::Yellow)))
        .child(TextView::new(text_art::TETRS_T).style(Color::Dark(BaseColor::Green)))
        .child(TextView::new(text_art::TETRS_R).style(Color::Dark(BaseColor::Cyan)))
        .child(TextView::new(text_art::TETRS_S).style(Color::Dark(BaseColor::Magenta)));

    let settings_button = get_settings_button();
    // settings holder
    let starting_score_container =
        HideableView::new(TextView::new("1").with_name(ids::STARTING_LEVEL)).hidden();

    let buttons = LinearLayout::vertical()
        .child(Button::new("Play", &play))
        .child(Button::new("Controls", |s| {
            controls_menu_popup(s);
        }))
        .child(settings_button)
        .child(get_quit_button());
    let title_view = OnEventView::new(
        Dialog::around(
            LinearLayout::vertical()
                .child(PaddedView::lrtb(0, 0, 1, 1, title_logo_view))
                .child(buttons)
                .child(starting_score_container),
        )
        .title("Tetrs | Rust Edition"),
    )
    .on_event(Event::Key(Key::Esc), |s| {
        s.quit();
    });
    s.add_layer(title_view);
}

fn play(siv: &mut Cursive) {
    siv.pop_layer();
    let high_score_label = TextView::new("High Score")
        .center()
        .style(Effect::Underline);
    let high_score = TextView::new("00000").center().with_name(ids::HIGH_SCORE);
    let score_label = TextView::new("Score").center().style(Effect::Underline);
    let score = TextView::new("00000").center().with_name(ids::SCORE);

    let lines_label = TextView::new("Lines").center().style(Effect::Underline);
    let lines = TextView::new("00000").center().with_name(ids::LINES);
    let level_label = TextView::new("Level").center().style(Effect::Underline);
    let level = TextView::new("00000").center().with_name(ids::LEVEL);

    let score_view = Dialog::around(
        LinearLayout::vertical()
            .child(high_score_label)
            .child(high_score)
            .child(score_label)
            .child(score)
            .child(lines_label)
            .child(lines)
            .child(level_label)
            .child(level),
    );
    let action_bubble =
        Dialog::around(TextView::new("...").center().with_name(ids::ACTION)).title("Last Action");
    let right_stack = Dialog::around(
        LinearLayout::vertical()
            .child(score_view)
            .child(Dialog::around(PieceView::new().with_name(ids::NEXT_PIECE)).title("Next Piece"))
            .child(
                HideableView::new(
                    Dialog::around(PieceView::new().with_name(ids::PIECE_IN_2)).title("In 2"),
                )
                .with_name(ids::HIDE_IN_2),
            )
            .child(
                HideableView::new(
                    Dialog::around(PieceView::new().with_name(ids::PIECE_IN_3)).title("In 3"),
                )
                .with_name(ids::HIDE_IN_3),
            )
            .child(
                HideableView::new(
                    Dialog::around(PieceView::new().with_name(ids::PIECE_IN_4)).title("In 4"),
                )
                .with_name(ids::HIDE_IN_4),
            ),
    )
    .title(" menu [esc] ")
    .title_position(cursive::align::HAlign::Center);

    let elapsed_view = LinearLayout::horizontal()
        .child(TextView::new("Elapsed: ").style(Effect::Underline))
        .child(
            TextView::new("00:00")
                .style(Effect::Underline)
                .with_name(ids::ELAPSED),
        );
    let singles_view = LinearLayout::horizontal()
        .child(TextView::new("Singles: ").style(Effect::Underline))
        .child(
            TextView::new("0")
                .style(Effect::Underline)
                .with_name(ids::SINGLES),
        );
    let doubles_view = LinearLayout::horizontal()
        .child(TextView::new("Doubles: ").style(Effect::Underline))
        .child(
            TextView::new("0")
                .style(Effect::Underline)
                .with_name(ids::DOUBLES),
        );
    let triples_view = LinearLayout::horizontal()
        .child(TextView::new("Triples: ").style(Effect::Underline))
        .child(
            TextView::new("0")
                .style(Effect::Underline)
                .with_name(ids::TRIPLES),
        );
    let tetrses_view = LinearLayout::horizontal()
        .child(TextView::new("Tetrses: ").style(Effect::Underline))
        .child(
            TextView::new("0")
                .style(Effect::Underline)
                .with_name(ids::TETRSES),
        );

    let tetrs_rate_view = LinearLayout::horizontal()
        .child(TextView::new("Tetrs Rate: ").style(Effect::Underline))
        .child(
            TextView::new("0%")
                .style(Effect::Underline)
                .with_name(ids::TETRS_RATE),
        );

    let stats_view = Dialog::around(
        LinearLayout::vertical()
            .child(elapsed_view)
            .child(singles_view)
            .child(doubles_view)
            .child(triples_view)
            .child(tetrses_view)
            .child(tetrs_rate_view),
    )
    .title("Stats");

    let left_stack = Dialog::around(
        LinearLayout::vertical()
            .child(Dialog::around(PieceView::new().with_name(ids::HELD_PIECE)).title("hold [c]"))
            .child(action_bubble)
            .child(stats_view),
    );

    let settings = BoardSettings {
        starting_level: get_starting_level(),
        ghost_piece_on: get_ghost_piece_on(),
        high_score: get_high_score(),
    };
    let board = Board::new(settings);
    siv.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::horizontal()
                    .child(left_stack)
                    .child(DummyView::new())
                    .child(board.with_name(ids::BOARD))
                    .child(DummyView::new())
                    .child(right_stack),
            )
            .title("Tetrs"),
        )
        .on_event(Event::Key(Key::Esc), |s| {
            pause_menu_popup(s);
        }),
    );
}

// helprs
fn pause_menu_popup(s: &mut Cursive) {
    s.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(Button::new("Resume", |s| {
                        s.pop_layer();
                    }))
                    .child(Button::new("Controls", |s| {
                        controls_menu_popup(s);
                    }))
                    .child(Button::new("Return to Title", |s| {
                        s.pop_layer();
                        s.pop_layer();
                        show_title_menu(s);
                    })),
            )
            .title("Pause Menu"),
        )
        .on_event(Event::Key(Key::Esc), |s| {
            s.pop_layer();
        }),
    );
}
fn controls_menu_popup(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(TextView::new(
            r#"
  <- / ->  | move piece left / right 
 up arrow  | instantly drop piece 
down arrow | fast fall piece
     z     | rotate piece left 
     x     | rotate piece right 
     c     | hold
"#,
        ))
        .dismiss_button("Close")
        .title("Controls"),
    );
}
fn get_quit_button() -> Button {
    Button::new("Quit", |s| {
        s.add_layer(
            Dialog::around(TextView::new("Are you sure you want to quit?"))
                .button("Yes", |s| {
                    s.quit();
                })
                .button("No", |s| {
                    s.pop_layer();
                })
                .title("Confirm Quit"),
        );
    })
}

fn get_ghost_piece_string() -> String {
    match get_ghost_piece_on() {
        true => "   On".to_string(),
        false => "  Off".to_string(),
    }
}

fn get_audio_on_off_string() -> String {
    match !audio::get_is_paused() {
        true => "         On".to_string(),
        false => "        Off".to_string(),
    }
}

pub fn get_settings_button() -> Button {
    Button::new("Settings", move |s| {
        let starting_level_button = Button::new("Change Starting Level", |s| {
            s.add_layer(
                // this is ugly, try to refactor
                OnEventView::new(
                    Dialog::around(TextView::new("Make selection:").center())
                        .button("1", |s| {
                            set_level(1);
                            s.pop_layer();
                        })
                        .button("2", |s| {
                            set_level(2);
                            s.pop_layer();
                        })
                        .button("3", |s| {
                            set_level(3);
                            s.pop_layer();
                        })
                        .button("4", |s| {
                            set_level(4);
                            s.pop_layer();
                        })
                        .button("5", |s| {
                            set_level(5);
                            s.pop_layer();
                        })
                        .button("6", |s| {
                            set_level(6);
                            s.pop_layer();
                        })
                        .button("7", |s| {
                            set_level(7);
                            s.pop_layer();
                        })
                        .button("8", |s| {
                            set_level(8);
                            s.pop_layer();
                        })
                        .button("9", |s| {
                            set_level(9);
                            s.pop_layer();
                        })
                        .title("Select a Level | ESC to close"),
                )
                .on_event(Event::Key(Key::Esc), |s| {
                    s.pop_layer();
                }),
            );
        });
        let toggle_ghost_piece_button = Button::new("Toggle Ghost Piece", |_s| {
            // toggle
            set_ghost_piece_on(!get_ghost_piece_on());
        });
        let toggle_audio_button = Button::new("Toggle Music", |_s| {
            // toggle
            let _ = audio::toggle();
        });
        s.add_layer(
            OnEventView::new(
                Dialog::around(
                    LinearLayout::vertical()
                        .child(DummyView)
                        .child(
                            LinearLayout::horizontal()
                                .child(starting_level_button)
                                .child(
                                    TextView::new(
                                        String::from(" ") + &get_starting_level().to_string(),
                                    )
                                    .with_name(ids::STARTING_LEVEL_PREVIEW),
                                ),
                        )
                        .child(LinearLayout::horizontal().child(toggle_audio_button).child(
                            TextView::new(get_audio_on_off_string()).with_name(ids::AUDIO_ON_OFF),
                        ))
                        .child(
                            LinearLayout::horizontal()
                                .child(toggle_ghost_piece_button)
                                .child(
                                    TextView::new(get_ghost_piece_string())
                                        .with_name(ids::GHOST_PIECE_ON_OFF),
                                ),
                        ),
                )
                .dismiss_button("Close")
                .title("Settings"),
            )
            .on_event(Event::Refresh, |s| {
                s.call_on_name(ids::STARTING_LEVEL_PREVIEW, |t: &mut TextView| {
                    t.set_content(String::from(" ") + &get_starting_level().to_string());
                });
                s.call_on_name(ids::GHOST_PIECE_ON_OFF, |t: &mut TextView| {
                    t.set_content(get_ghost_piece_string());
                });
                s.call_on_name(ids::AUDIO_ON_OFF, |t: &mut TextView| {
                    t.set_content(get_audio_on_off_string());
                });
            })
            .on_event(Event::Key(Key::Esc), |s| {
                s.pop_layer();
            }),
        );
    })
}
//helpers
fn get_high_score_from_disk() -> u32 {
    match save::read_config() {
        Ok(hs_str) => match hs_str.parse::<u32>() {
            Ok(x) => x,
            Err(_) => 0,
        },
        Err(_) => 0,
    }
}
fn save_high_score_to_disk(score: u32) {
    let _ = save::write_config(&score.to_string());
}
