/// Design an algorithm to encode a list of strings to a string. The encoded string is then sent
/// over the network and is decoded back to the original list of strings.
struct Codec; 

impl Codec {

    fn new() -> Self {
        Self { }
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let n = strs.len() as u32;
        let n_char = char::from_u32(n).unwrap();
        let mut result = String::from(n_char);

        for str in strs {
            let len = str.len() as u32;
            let len_char = char::from_u32(len).unwrap().to_string();
            result = result + &len_char + &str;
        }

        result
    }

    fn decode(&self, s: String) -> Vec<String> {
        let components: Vec<char> = s.chars().collect();
        let n = (components[0] as u32) as usize;
        let mut result = Vec::with_capacity(n);

        let mut index = 1;
        
        for _ in 0..n {
            let length = (components[index] as u32) as usize;
            index += 1;
            let s: String = components[index..index+length].iter().collect();
            result.push(s);
            index += length;
        }

        result
    } 

}

#[cfg(test)]
mod tests {
    use super::Codec;

    #[test]
    fn example_1() {
        let input = vec!["Hello", "World"];
        let input: Vec<String> = input.into_iter().map(|s| s.to_string()).collect();
        let codec = Codec::new();
        let encoded = codec.encode(input);
        let result = codec.decode(encoded);
        assert_eq!(result, vec!["Hello", "World"]);
    }

    #[test]
    fn example_2() {
        let input = vec!["".to_string()];
        let codec = Codec::new();
        let encoded = codec.encode(input);
        let result = codec.decode(encoded);
        assert_eq!(result, vec![""]);
    }

}
