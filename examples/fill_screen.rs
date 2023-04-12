use farba::{Canvas, RGBAColor};

fn main() {
    let mut canvas = Canvas::new(400, 400);

    canvas.fill(RGBAColor::RED);

    canvas.save_to_file("./examples/fill_screen.png")
}
