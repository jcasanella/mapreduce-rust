pub trait Mapper {
    type Input;
    type Key;
    type Value;

    fn map(&self, input: Self::Input) -> Vec<(Self::Key, Self::Value)>;
}

pub struct WordCountMapper;

impl Mapper for WordCountMapper {
    type Input = String;
    type Key = String;
    type Value = u32;

    fn map(&self, input: Self::Input) -> Vec<(Self::Key, Self::Value)> {
        let mut word_count = std::collections::HashMap::new();

        println!("Mapping input: {}", input);
        for word in input.split_whitespace() {
            *word_count.entry(word.to_string()).or_insert(0) += 1;
        }
        word_count.into_iter().collect()
    }
}