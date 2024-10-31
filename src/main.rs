// 1. Run this program as it is and observe the output.
// 2. Implement the Describable trait for both the Video and Book structs.
// 3. After implementing the Describable trait, use it to call the describe method on instances of Video and Book to display custom descriptions.

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Video {
    pub title: String,
    pub duration: u32, // duração em minutos
    pub content: String,
}

fn main() {
    let book = Book {
        title: String::from("O Senhor dos Anéis"),
        author: String::from("J.R.R. Tolkien"),
        content: String::from("Conteúdo do livro..."),
    };

    let video = Video {
        title: String::from("Rust Programming Tutorial"),
        duration: 120,
        content: String::from("Conteúdo do vídeo..."),
    };

    println!("Novo item: {:#?}", book);
    println!("Novo item: {:#?}", video);
}
