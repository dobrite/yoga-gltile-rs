use renderer;
use yoga;

pub struct Backend {
    renderer: renderer::Renderer,
}

impl Backend {
    pub fn new() -> Backend {
        Backend { renderer: renderer::Renderer::new() }
    }
}

impl<'meas> yoga::Backend<'meas> for Backend {
    type Renderer = renderer::Renderer;

    fn get_renderer(&mut self) -> &mut renderer::Renderer {
        &mut self.renderer
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
