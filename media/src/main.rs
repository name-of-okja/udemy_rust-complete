#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcase(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {} ", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcase(episode_number) => {
                format!("Podcast episode: {}", episode_number)
            }
            Media::Placeholder => String::from("Placeholder"),
        }
    }
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("The Rust Programming Language"),
    };
    let good_movie = Media::Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
    };
    let bad_book = Media::Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
    };
    let podcast = Media::Podcase(42);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(40);

    println!("{:?}", item.expect("Not found"));
}
