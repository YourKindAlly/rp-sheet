/**
 * @license MIT License
 * @author Jasmine Regnér
 */

#[cfg(test)]
mod config_tests {
    use crate::config::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_path_exists() {
        let path = Path::new("testdirs/user");
        let result = is_existing_path(&path);
        assert!(result);

        let path = Path::new("testdirs/empty");
        let result = is_existing_path(&path);
        assert!(result == false);
    }

    #[test]
    fn test_create_config() {
        let path = Path::new("testdirs/user");
        let result = create_config_directory(&path).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn test_file_name() {
        let path = Path::new("testdirs/user");
        let result = create_config_file_name(path);
        assert_eq!(result, PathBuf::from("testdirs/user/config.json"));
    }

    #[test]
    fn test_create_config_contents() {
        let path = Path::new("testdirs/user");
        let config = create_config_contents(path);
        let config_template_path = PathBuf::from("testdirs/user/templates");
        assert_eq!(config.sheet_template_dir, config_template_path);
    }

    #[test]
    fn test_create_json() {
        let path = Path::new("testdirs/user");
        let config = create_config_contents(path);
        let json = create_json(&config);
        let comparison = String::from("{\"sheet_template_dir\":\"testdirs/user/templates\"}");
        assert_eq!(json, comparison)
    }

    #[test]
    fn test_write_to_config_file() {
        let config_path = PathBuf::from("testdirs/user/config.json");
        let template_path = Path::new("testdirs/user/templates");
        let config = create_config_contents(&template_path);
        let json = create_json(&config);

        write_to_config_file(&config_path, &json);
    }

    #[test]
    #[should_panic]
    fn test_panic_write_to_config_file() {
        let config_path = PathBuf::from("testdirs/admin/config.json");
        let template_path = Path::new("testdirs/admin/templates");
        let config = create_config_contents(&template_path);
        let json = create_json(&config);

        write_to_config_file(&config_path, &json);
    }
}
