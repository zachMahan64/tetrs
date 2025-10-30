use crate::board::Board;
use crate::text_art;
use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::event::Event;
use cursive::event::Key;
use cursive::theme::BaseColor;
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
        .child(Button::new("Quit", &Cursive::quit));
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
    let quit_button = PaddedView::lrtb(
        5,
        5,
        10,
        0,
        Button::new("Quit", |s| {
            s.pop_layer();
            show_title_menu(s);
        }),
    );
    let score_label = TextView::new("Score:").center();
    let score = TextView::new("0").with_name("score");
    let action_bubble = TextView::new("...").with_name("action");
    let side_stack = Dialog::around(
        LinearLayout::vertical()
            .child(score_label)
            .child(score)
            .child(DummyView::new())
            .child(action_bubble)
            .child(quit_button),
    );
    let board = Board::new(); // TODO: pass settings here, eventually
    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(board.with_name("board"))
                .child(DummyView::new())
                .child(side_stack),
        )
        .title("Tetrs"),
    );
    // tetrs.siv.focus_name("board").unwrap();
}
