error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "Error during I/O operation"];
        Http(::hyper::Error) #[doc = "Networking error"];
        Format(::serde_json::error::Error) #[doc = "(De)serialization error"];
    }
}