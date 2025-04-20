fn main() {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rusty SpaceGame", SCREEN_SIZE, SCREEN_SIZE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
}
