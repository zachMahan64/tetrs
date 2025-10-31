use crate::board::Board;
use crate::text_art;
use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::event::Event;
use cursive::event::Key;
use cursive::theme::BaseColor;
use cursive::theme::Effect;
use cursive::traits::*;
use cursive::views::DummyView;
use cursive::views::OnEventView;
use cursive::views::PaddedView;
use cursive::views::TextView;
use cursive::views::{Button, Dialog, LinearLayout, SelectView};
pub fn run() {
    let mut siv = CursiveRunnable::default();
    let mut theme = siv.current_theme().clone();
    theme.palette = cursive::theme::Palette::retro();
    siv.set_theme(theme);
    // enter title menu
    show_title_menu(&mut siv);
    // init cursive
    siv.set_fps(60);
    siv.run();
}

pub fn show_title_menu(s: &mut Cursive) {
    let select = SelectView::<String>::new().with_name("select");
    use cursive::theme::Color;
    let title_logo_view = LinearLayout::horizontal()
        .child(TextView::new(text_art::TETRS_T).style(Color::Dark(BaseColor::Red)))
        .child(TextView::new(text_art::TETRS_E).style(Color::Dark(BaseColor::Yellow)))
        .child(TextView::new(text_art::TETRS_T).style(Color::Dark(BaseColor::Green)))
        .child(TextView::new(text_art::TETRS_R).style(Color::Dark(BaseColor::Cyan)))
        .child(TextView::new(text_art::TETRS_S).style(Color::Dark(BaseColor::Magenta)));
    let settings_button = Button::new("Settings", |s| {
        s.add_layer(
            Dialog::around(TextView::new("Sorry, there's nothing to set just yet."))
                .button("Cancel", |s| {
                    s.pop_layer();
                })
                .title("Settings"),
        );
    });

    let buttons = LinearLayout::vertical()
        .child(Button::new("Play", &play))
        .child(settings_button)
        .child(quit_button());
    let title_view = OnEventView::new(
        Dialog::around(
            LinearLayout::vertical()
                .child(select)
                .child(title_logo_view)
                .child(DummyView::new())
                .child(buttons),
        )
        .title("Tetrs / Rust Edition | By Zach Mahan"),
    )
    .on_event(Event::Key(Key::Esc), |s| {
        s.quit();
    });
    s.add_layer(title_view);
}

fn play(siv: &mut Cursive) {
    siv.pop_layer();
    let pause_button = PaddedView::lrtb(5, 5, 10, 0, pause_button());
    let high_score_label = TextView::new("High Score")
        .center()
        .style(Effect::Underline);
    let high_score = TextView::new("1000").center().with_name("highscore"); // TODO add logic
    let score_label = TextView::new("Score").center().style(Effect::Underline);
    let score = TextView::new("0").center().with_name("score"); // TODO add logic
    let score_view = Dialog::around(
        LinearLayout::vertical()
            .child(high_score_label)
            .child(high_score)
            .child(score_label)
            .child(score),
    );
    let action_bubble = TextView::new("...").with_name("action");
    let side_stack = Dialog::around(
        LinearLayout::vertical()
            .child(score_view)
            .child(DummyView::new())
            .child(action_bubble)
            .child(pause_button),
    );
    let board = Board::new(); // TODO: pass settings here, eventually
    siv.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::horizontal()
                    .child(board.with_name("board"))
                    .child(DummyView::new())
                    .child(side_stack),
            )
            .title("Tetrs"),
        )
        .on_event(Event::Key(Key::Esc), |s| {
            pause_menu(s);
        }),
    );
    // tetrs.siv.focus_name("board").unwrap();
}

// helprs
fn pause_button() -> Button {
    Button::new("Pause", |s| {
        pause_menu(s);
    })
}
fn pause_menu(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(Button::new("Resume", |s| {
                    s.pop_layer();
                }))
                .child(Button::new("Return to Title", |s| {
                    s.pop_layer();
                    s.pop_layer();
                    show_title_menu(s);
                })),
        )
        .title("Pause Menu"),
    );
}
fn quit_button() -> Button {
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
