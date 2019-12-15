pub mod intcode;

#[macro_export]
macro_rules! input {
    () => {{
        use std::io::Read;

        let file_name = std::file!();
        let mut file_name = std::path::PathBuf::from(file_name);
        file_name.set_extension("txt");

        let mut buffer = String::new();
        std::fs::File::open(&file_name)
            .unwrap_or_else(|e| panic!("Could not open {:?}: {:?}", file_name, e))
            .read_to_string(&mut buffer)
            .unwrap_or_else(|e| panic!("Could not read {:?}: {:?}", file_name, e));

        buffer
    }};
}
