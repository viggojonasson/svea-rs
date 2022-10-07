use crate::path::Path;
use crate::server::Handler;

pub struct Routes(Vec<(Path, Handler)>);

impl Routes {
    pub fn new() -> Self {
        Routes(Vec::new())
    }

    pub fn add(&mut self, path: Path, handler: Handler) {
        self.0.push((path, handler));
    }

    pub fn find_matching(&self, path: Path) -> Option<(Path, Handler)> {
        for (p, h) in self.0 {
            if p == path {
                return Some((p, h));
            }
        }
        None
    }
}
