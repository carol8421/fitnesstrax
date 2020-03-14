macro_rules! enclose {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(enclose!(@param $p),)+| $body
        }
    );
}

use gtk::prelude::*;

mod basics;
mod date_selector;
mod day;
mod history;
mod main_window;
mod range_selector;
mod rep_duration;
mod set_rep;
mod settings;
mod steps;
mod time_distance;
mod time_distance_row;
mod weight;

pub use basics::*;
pub use date_selector::date_selector_c;
pub use day::Day;
pub use history::History;
pub use main_window::MainWindow;
pub use range_selector::RangeSelector;
pub use settings::Settings;

pub trait Component {
    fn widget(&self) -> gtk::Widget;
}

impl Component for gtk::Widget {
    fn widget(&self) -> gtk::Widget {
        self.clone()
    }
}

impl Component for gtk::Box {
    fn widget(&self) -> gtk::Widget {
        self.clone().upcast::<gtk::Widget>()
    }
}
