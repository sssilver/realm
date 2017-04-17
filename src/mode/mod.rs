pub mod menu;


use piston_window::{
    AfterRenderArgs,
    Button,
    IdleArgs,
    RenderArgs,
    UpdateArgs
};


pub trait Mode {
    fn after_render(&self, after_render_args: &AfterRenderArgs) { }
    fn cursor(&self, cursor_args: bool) { }
    fn focus(&self, focus_args: bool) { }
    fn idle(&self, idle_args: &IdleArgs) { }
    fn press(&self, press_args: Button) { }
    fn release(&self, release_args: Button) { }
    fn render(&self, render_args: &RenderArgs) { }
    fn resize(&self, resize_args: [u32; 2]) { }
    fn text(&self, text_args: String) { }
    fn update(&self, update_args: &UpdateArgs) { }

    fn run(&self, event: i64) {
        //self.render();
    }
}
