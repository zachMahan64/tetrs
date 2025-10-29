use crate::text_art;
use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::traits::*;
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
    log_err!("started cursive");
}

fn show_title_menu(s: &mut Cursive) {
    let select = SelectView::<String>::new().with_name("select");
    let mut title_logo_view = TextView::new(text_art::TETRS_LOGO_BLOCK);
    title_logo_view.set_style(cursive::theme::Color::Rgb(0, 0, 0));
    let buttons = LinearLayout::vertical()
        .child(Button::new("Play", &play))
        .child(Button::new("Quit", &Cursive::quit));
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(select)
                .child(title_logo_view)
                .child(buttons),
        )
        .title("Tetrs: Rust Edition, Pun Intented | By Zach Mahan"),
    );
}

struct Tetrs<'a> {
    siv: &'a mut Cursive,
    // TODO: add a gamefield, leaderboard
}

impl<'a> Tetrs<'a> {
    fn new(siv: &'a mut Cursive) -> Self {
        Self { siv: siv }
    }
}

fn play(s: &mut Cursive) {
    s.pop_layer();
    let tetrs = Tetrs::new(s);
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(TextView::new("Test  "))
                .child(Button::new("Cancel", |s| {
                    s.pop_layer();
                    show_title_menu(s);
                })),
        )
        .title("Tetrs"),
    );
    s.add_global_callback('q', |s| {
        s.pop_layer();
        show_title_menu(s);
    });
}

/*
// TODO: example code
fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        s.pop_layer();
    }

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("name")
                .fixed_width(10),
        )
        .title("Enter a new name")
        .button("Ok", |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}

// TODO: example code
fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(format!("Name: {}\nAwesome: yes", name))
            .title(format!("{}'s info", name))
            .button("Quit", Cursive::quit),
    );
}
*/
