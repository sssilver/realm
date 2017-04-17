use mode::*;
use piston::input::*;


pub struct Game<'a> {
    pub mode: &'a (Mode + 'a)
}


impl<'a> Game<'a> {
    fn run(&self) {

    }

    pub fn dispatch(&self, event: &Event) {
        if let Some(u) = event.after_render_args() {
            self.mode.after_render(&u);
        }

        if let Some(u) = event.cursor_args() {
            self.mode.cursor(u);
        }

        if let Some(u) = event.focus_args() {
            self.mode.focus(u);
        }

        if let Some(u) = event.idle_args() {
            self.mode.idle(&u);
        }

        if let Some(u) = event.press_args() {
            self.mode.press(u);
        }

        if let Some(u) = event.release_args() {
            self.mode.release(u);
        }

        if let Some(u) = event.render_args() {
            self.mode.render(&u);
        }

        if let Some(u) = event.resize_args() {
            self.mode.resize(u);
        }

        if let Some(u) = event.text_args() {
            self.mode.text(u);
        }

        if let Some(u) = event.update_args() {
            self.mode.update(&u);
        }


        //println!("{:?}", event);
    }
}
