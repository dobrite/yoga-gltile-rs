extern crate glium;
extern crate gltile;

use glium::DisplayBuild;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1536, 1024)
        .build_glium()
        .unwrap();

    let mut renderer = gltile::Renderer::new(&display, 16, "examples/assets/tileset.png");

    let tile = {
        let mut tile = gltile::Tile::new();
        tile.fg = *gltile::colors::YELLOW;
        tile.pix = gltile::Pix::Dood;
        tile
    };

    renderer.set(gltile::units::ScreenTile2D::new(5, 5), tile);

    loop {
        renderer.render();

        for ev in renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
