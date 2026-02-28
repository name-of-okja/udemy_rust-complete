mod content;

use content::catalog::Catalog;
use content::media::Media;

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
