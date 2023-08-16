use guess_word::Dictionary;

fn main() {
    println!("{}", Dictionary::default().get_random_word());
}
