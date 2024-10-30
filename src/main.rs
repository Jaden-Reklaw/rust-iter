fn colors() -> Vec<String> {
    vec![
        String::from("Red"),
        String::from("Green"),
        String::from("Blue"),
        String::from("Yellow"),
        String::from("Orange"),
        String::from("Purple"),
        String::from("Black"),
        String::from("White"),
        String::from("Pink"),
        String::from("Brown"),
    ]
}

fn print_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn print_elements_consumer(elements: &Vec<String>) {
    elements
    .iter()
    .for_each(|element| println!("{}", element));
}

fn print_elements_iterator_adaptor_upper(elements: &Vec<String>) {
    elements.iter()
    .map(|element| element.to_uppercase())
    .for_each(|element| println!("{}", element));
}

//Note the difference in the signature of the function below
//The function below takes a slice of the vector which allows the entire or a portion of a
//vector to be passed to the function
fn print_elements_iterator_adaptor_lower(elements: &[String]) {
    elements.iter()
    .map(|element| element.to_lowercase())
    .for_each(|element| println!("{}", element));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el|el.truncate(1));
}

fn colors_to_uppercase(elements: &mut [String]) -> Vec<String> {
    elements
    .iter()
    .map(|el|el.to_uppercase())
    .collect::<Vec<String>>()
}

//Take ownership using into_iter
fn move_elements(a: Vec<String>, b: &mut Vec<String>) {
    b.extend(a.into_iter());

    //or
    // a.into_iter().for_each(|el|b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
    .iter()
    .map(|el|el
        .chars()
        .map(|c|c.to_string())
        .collect()
    )
    .collect()
}

fn search_for_color(elements: &[String], search: &str, default: &str) -> String {
    elements
    .iter()
    .find(|el|el.contains(search))
    .map_or(default.to_string(), |el|el.to_string())
}

fn main() {
    // create vector of all colors
    let mut all_colors = colors();
    // print_elements(&all_colors);
    // print_elements_consumer(&all_colors);
    // print_elements_iterator_adaptor_upper(&all_colors);
    // print_elements_iterator_adaptor_lower(&all_colors[1..3]);
    
    // shorten_strings(&mut all_colors[1..4]);
    // print_elements(&all_colors);

    // let upper_colors = colors_to_uppercase(&mut all_colors[1..4]);
    // print_elements(&upper_colors);
    
    // let mut dest = Vec::new();
    // move_elements(all_colors, &mut dest);
    // println!("Destination vector: {:#?}", dest);

    // let exploded = explode(&all_colors);
    // println!("{:#?}", exploded);

    let found_color = search_for_color(&all_colors, "Bl", "Black");
    println!("{:#?}", found_color);
}
