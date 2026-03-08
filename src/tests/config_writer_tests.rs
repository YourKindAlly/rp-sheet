/**
 * @license MIT License
 * @author Jasmine Regnér
 */

#[cfg(test)]
mod config_tests {
    use crate::config_writer::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_create_config() {
        let path = PathBuf::from("testdirs/user");
        let result = create_config_directory(&path).unwrap();
        assert_eq!(result, ());

        let path = PathBuf::from("testdirs/admin");
        let result = create_config_directory(&path).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn test_file_name() {
        let path = Path::new("testdirs/user");
        let result = create_config_file_name(path);
        assert_eq!(result, PathBuf::from("testdirs/user/config.json"));
    }
}
