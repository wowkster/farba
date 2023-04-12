use farba::{Canvas, RGBAColor};

const CANVAS_WIDTH: usize = 900;
const CANVAS_HEIGHT: usize = 600;

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Taken from https://upload.wikimedia.org/wikipedia/en/9/9e/Flag_of_Japan.svg
    canvas.fill(RGBAColor::WHITE);
    canvas.circle(
        (CANVAS_WIDTH / 2) as i32,
        (CANVAS_HEIGHT / 2) as i32,
        180,
        RGBAColor::from_rgb(0xBC, 0x00, 0x2D),
    );

    canvas.save_to_file("./examples/flag_of_japan.png");
}
