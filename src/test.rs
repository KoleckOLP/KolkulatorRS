return match key_result {
    Ok(key) => {
        let output = match key {
            console_utils::read::Key::Char(c) => String::from(c),
            other => format!("{:?}", other),
        };
        output
    }
    Err(err) => {
        panic!("Failed to read_key, {err}");
    }
};

fn _type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}