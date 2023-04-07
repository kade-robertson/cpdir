use std::{collections::VecDeque, fs::read_dir, path::PathBuf};

pub struct WalkableDir {
    depth: u8,
    queue: VecDeque<(PathBuf, u8)>,
}

fn push_subdirectories(path: &PathBuf, depth: u8, queue: &mut VecDeque<(PathBuf, u8)>) {
    if let Ok(dir_contents) = read_dir(path) {
        for entry in dir_contents.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                queue.push_back((entry_path, depth + 1));
            }
        }
    }
}

impl WalkableDir {
    pub fn new(path: &PathBuf, depth: u8) -> Self {
        let mut dir_queue = VecDeque::new();
        if depth > 0 {
            push_subdirectories(path, 0, &mut dir_queue);
        }
        WalkableDir {
            depth,
            queue: dir_queue,
        }
    }
}

impl Iterator for WalkableDir {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let (current_dir, current_depth) = match self.queue.pop_front() {
            Some(dir) => dir,
            None => return None,
        };

        if current_depth < self.depth {
            push_subdirectories(&current_dir, current_depth, &mut self.queue);
        }

        Some(current_dir)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir;

    use super::*;
    use mktemp::Temp;

    #[test]
    fn empty_iterator_for_empty_dir() {
        let dir = Temp::new_dir().unwrap();
        let walkable = WalkableDir::new(&dir, 255);

        assert_eq!(walkable.count(), 0);
    }

    #[test]
    fn all_top_level_subdirs_for_dir() {
        let dir = Temp::new_dir().unwrap();

        assert!(create_dir(dir.join("fake1")).is_ok());
        assert!(create_dir(dir.join("fake2")).is_ok());
        assert!(create_dir(dir.join("fake3")).is_ok());

        let walkable = WalkableDir::new(&dir, 255);

        assert_eq!(walkable.count(), 3);
    }

    #[test]
    fn skips_files_in_dir() {
        let dir = Temp::new_dir().unwrap();

        assert!(Temp::new_file_in(&dir).is_ok());
        assert!(Temp::new_file_in(&dir).is_ok());
        assert!(Temp::new_file_in(&dir).is_ok());
        assert!(create_dir(dir.join("fake1")).is_ok());
        assert!(create_dir(dir.join("fake2")).is_ok());
        assert!(create_dir(dir.join("fake3")).is_ok());

        let walkable = WalkableDir::new(&dir, 255);

        assert_eq!(walkable.count(), 3);
    }

    #[test]
    fn empty_iterator_for_zero_depth_dir() {
        let dir = Temp::new_dir().unwrap();

        assert!(create_dir(dir.join("fake1")).is_ok());
        assert!(create_dir(dir.join("fake2")).is_ok());
        assert!(create_dir(dir.join("fake3")).is_ok());

        let walkable = WalkableDir::new(&dir, 0);

        assert_eq!(walkable.count(), 0);
    }

    #[test]
    fn respects_depth_limit_for_dir() {
        let dir = Temp::new_dir().unwrap();

        assert!(create_dir(dir.join("fake1")).is_ok());
        assert!(create_dir(dir.join("fake2")).is_ok());
        assert!(create_dir(dir.join("fake3")).is_ok());
        assert!(create_dir(dir.join("fake1").join("fake1")).is_ok());
        assert!(create_dir(dir.join("fake1").join("fake2")).is_ok());
        assert!(create_dir(dir.join("fake1").join("fake3")).is_ok());

        assert_eq!(WalkableDir::new(&dir, 255).count(), 6);
        assert_eq!(WalkableDir::new(&dir, 1).count(), 3);
    }
}
