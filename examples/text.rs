extern crate glium;
extern crate gltile;
extern crate pixset;
extern crate yoga;
extern crate yoga_gltile;

use glium::DisplayBuild;
use yoga::{Backend, Builds, Renderable};

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1536, 1024)
        .build_glium()
        .unwrap();

    let pixset = pixset::Pixset::new(100, 16);
    let mut renderer = gltile::Renderer::new(&display, pixset);

    let builder = yoga_gltile::Builder::new();

    let mut text = builder.text("Yo Dawg");
    text.set_color(Some(*gltile::colors::WHITE));

    let mut text2 = builder.text("Word Up");
    text2.set_color(Some(*gltile::colors::YELLOW));

    let mut root = builder.view();
    root.set_width(50.0);
    root.set_height(12.0);
    root.set_flex_direction(yoga::FlexDirection::Row);
    root.set_padding(yoga::Edge::All, 2.0);
    root.set_background_color(Some(yoga::style::BackgroundColor::Color(*gltile::colors::CYAN)),);

    root.insert_child(&text, 0);
    root.insert_child(&text2, 0);

    root.calculate_layout();

    let mut console = gltile::Console::new(renderer.size);
    let mut be = yoga_gltile::Backend::new();

    loop {
        console = be.render(&root, console);
        renderer.blit_console(gltile::units::ScreenTile2D::new(0, 0), &console);
        renderer.render();

        for ev in renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
