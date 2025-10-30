use crate::board::Board;
use crate::text_art;
use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::event::Key;
use cursive::theme::BaseColor;
use cursive::traits::*;
use cursive::views::DummyView;
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

    let buttons = LinearLayout::vertical()
        .child(Button::new("Play", &play))
        .child(Button::new("Quit", &Cursive::quit));
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(select)
                .child(title_logo_view)
                .child(DummyView::new())
                .child(buttons),
        )
        .title("Tetrs: Rust Edition, Pun Intented | By Zach Mahan"),
    );
}

struct Tetrs<'a> {
    siv: &'a mut Cursive,
    board: Board,
}

impl<'a> Tetrs<'a> {
    fn new(siv: &'a mut Cursive) -> Self {
        Self {
            siv: siv,
            board: Board::new(),
        }
    }
}

fn play(siv: &mut Cursive) {
    siv.pop_layer();
    let tetrs = Tetrs::new(siv);
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
    let side_stack = PaddedView::lrtb(
        2,
        2,
        0,
        0,
        LinearLayout::vertical()
            .child(score_label)
            .child(score)
            .child(DummyView::new())
            .child(action_bubble)
            .child(quit_button),
    );

    tetrs.siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(tetrs.board.with_name("board"))
                .child(DummyView::new())
                .child(side_stack),
        )
        .title("Tetrs"),
    );
    // tetrs.siv.focus_name("board").unwrap();
}
