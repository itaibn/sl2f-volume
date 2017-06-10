extern crate gtk;
extern crate modinverse;

use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;

use modinverse::modinverse;

const MODULUS: i32 = 10001;

#[derive(Debug)]
struct State {
    volume: Cell<i32>,
    volume_label: gtk::Label,
}

impl State {
    fn new(volume_label: gtk::Label) -> Self {
        let res = State {
            volume: Cell::new(0),
            volume_label,
        };
        res.update_label();
        res
    }

    fn update_label(&self) {
        let volume = self.volume.get();
        let new_label = format!("Volume: {}.{:02}%", volume / 100,
            volume % 100);
        self.volume_label.set_label(&new_label);
    }

    // WARNING: Do not use directly. You should only access this with plus_one()
    // and invert().
    fn set_volume(&self, new_volume: i32) {
        self.volume.set(new_volume);
        self.update_label();
    }

    fn plus_one(&self) {
        self.set_volume((self.volume.get() + 1) % MODULUS);
    }

    fn invert(&self) {
        let mut inverse = modinverse(self.volume.get(), MODULUS).expect(
            "What, you expected this GUI to have decent error handling?");
        // modinverse doesn't specify the range of its output and sometimes
        // returns negative numbers. Ad-hoc fix:
        inverse = inverse % MODULUS;
        if inverse < 0 {
            inverse += MODULUS;
        }
        self.set_volume(inverse);
    }
}

fn main() {
    gtk::init().expect("initializing glade");

    let glade_src = include_str!("sl2f-volume.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let top_window: gtk::Window = builder.get_object("top-window").unwrap();
    top_window.show_all();
    top_window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    let state =
        Rc::new(State::new(builder.get_object("volume-label").unwrap()));

    let plus_one_button: gtk::Button = builder.get_object("plus-one").unwrap();
    let state_clone = state.clone();
    plus_one_button.connect_clicked(move |_| {
        state_clone.plus_one();
    });

    let invert_button: gtk::Button = builder.get_object("invert").unwrap();
    invert_button.connect_clicked(move |_| {
        state.invert();
    });

    gtk::main();
}
