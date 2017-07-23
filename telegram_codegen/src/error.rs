error_chain! {
    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);

        SerdeJson(::serde_json::Error);
    }
}
