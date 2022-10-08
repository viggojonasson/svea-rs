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

    pub fn find_matching_handler(&self, path: &Path) -> Option<&Handler> {
        for (p, h) in &self.0 {
            if p.queries.0.len() == 0 {
                println!("Comparing: {} = {}", p.path, path.path);
                if p.path == path.path {
                    return Some(&h);
                }
            }

            if p == path {
                return Some(&h);
            }
        }
        None
    }
}
