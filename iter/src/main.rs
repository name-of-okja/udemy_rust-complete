fn print_all(elements: &Vec<String>) {
    // for el in elements {
    //     println!("{}", el);
    // }

    // elements.iter().for_each(|el| println!("{}", el));

    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<_>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("Red"),
        String::from("Green"),
        String::from("Blue"),
    ];

    // shorten_strings(&mut colors);
    // print_all(&colors);

    // let uppercased = to_uppercase(&colors);
    // print_all(&uppercased);

    // let mut dest = vec![];
    // move_elements(colors, &mut dest);
    // println!("Dest: {:#?}", dest);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded);

    let found_color = find_color_or(&colors, "re", "Black");
    println!("{:#?}", found_color);
}
