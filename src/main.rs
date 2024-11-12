use proc_macro::Builder;

fn main() {
    #[derive(Debug, Builder)]
    struct Animal {
        name: String,
        age: u8,
    }

    let a: Result<Animal, Box<dyn std::error::Error>> = AnimalBuilder::builder()
        .name("alex".to_string())
        .age(29)
        .build();

    println!("{:#?}", a);
}
