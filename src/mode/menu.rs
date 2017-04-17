use piston_window::{RenderArgs, UpdateArgs};


pub struct Menu {
    pub title: String
}


impl ::mode::Mode for Menu {
    fn render(&self, render_args: &RenderArgs) {
        //println!("render: {:?}", render_args);
    }

    fn update(&self, update_args: &UpdateArgs) {
        //println!("update: {:?}", update_args);
    }
}
