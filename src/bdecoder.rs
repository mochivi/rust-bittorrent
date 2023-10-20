pub mod bdecoder {
    use serde_json::{Number, Value, Map};

    // Get the next matching value
    pub fn decode(mut encoded_string: &str) -> Vec<Value> {
        //let encoded_string: &str = &encoded_string;
        let mut decoded_array: Vec<Value> = Vec::new();

        loop {
            println!("\nDecoding next: {}", encoded_string);
            if encoded_string.len() == 1 {
                break;
            }
            match encoded_string.chars().next() {
                Some(c) if c.is_digit(10) => {
                    println!("String char: {}", encoded_string);
                    let (decoded_string, next_idx) = decode_string(&encoded_string);
                    println!("string: {}", decoded_string);
                    decoded_array.push(decoded_string);
                    let (new_string, is_end) = consume_string(&encoded_string, next_idx);
                    println!("New string: {}", new_string);
                    encoded_string = new_string;
                    if is_end {
                        break;
                    }
                },
                Some('i') => {
                    println!("Integer char: {}", encoded_string);
                    let (decoded_integer, next_idx) = decode_integer(&encoded_string);
                    println!("integer: {}", decoded_integer);
                    decoded_array.push(decoded_integer);
                    let (new_string, is_end) = consume_string(&encoded_string, next_idx);
                    println!("New string: {}", new_string);
                    encoded_string = new_string;
                    if is_end {
                        break;
                    }
                },
                Some('l') => {
                    let (decoded_list, next_idx) = decode_list(&encoded_string);
                    decoded_array.push(decoded_list);
                    let (new_string, is_end) = consume_string(&encoded_string, next_idx);
                    println!("New string: {}", new_string);
                    encoded_string = new_string;
                    if is_end {
                        break;
                    }
                },
                Some('d') => {
                    let (decoded_dict, next_idx) = decode_dict(encoded_string);
                    decoded_array.push(decoded_dict);
                    let (new_string, is_end) = consume_string(&encoded_string, next_idx);
                    println!("New string: {}", new_string);
                    encoded_string = new_string;
                    if is_end {
                        break;
                    }
                },
                Some('e') => {
                    println!("End char: {}", encoded_string);
                    break;
                },
                None => {},
                _ => panic!("Unhandled encoded value")
            }
        }

        return decoded_array;
    }

    fn decode_next(encoded_string: &str) -> Option<(serde_json::Value, usize)> {
        println!("Decoding next (sequence): {}", encoded_string);

        match encoded_string.chars().next() {
            Some(c) if c.is_digit(10) => {
                let (value, next_idx) = decode_string(encoded_string);
                return Some((value, next_idx));
            },
            Some('i') => {
                let (value, next_idx) = decode_integer(encoded_string);
                return Some((value, next_idx));
            },
            Some('l') => {
                let (value, next_idx) = decode_list(encoded_string);
                return Some((value, next_idx));
            },
            Some('d') => {
                let (value, next_idx) = decode_dict(encoded_string);
                return Some((value, next_idx));
            },
            Some('e') => {
                println!("Finished sequence\n");
                return None;
            },
            _ => panic!("Unexpected character")
        }
    }
    
    fn consume_string(encoded_string: &str, next_idx: usize) -> (&str, bool) {
        println!("Consuming encoded_string: {}", encoded_string);
        let new_string: &str = &encoded_string[next_idx..];
        if new_string.len() == 1 {
            // Consumed string entirely, only 'e' left.
            println!("{}", new_string);
            return (new_string, true);
        }
        println!("Consumed to: {}", new_string);
        return (new_string, false);
    }

    fn decode_dict(encoded_string: &str) -> (serde_json::Value, usize) {
        println!("\nDict char: {}", encoded_string);

        let mut dict: Map<String, Value> = Map::new();

        // Starting from index '1'
        let mut dict_string: &str = encoded_string.clone();
        dict_string = &dict_string[1..];
        let mut step: usize = 1; // + beginning and end characters for dict pattern

        loop {
            if let Some((key, next_idx)) = decode_next(dict_string) {
                // Continue from the next token
                dict_string = &dict_string[next_idx..];
                
                let key_print = key.clone();

                if let Some((value, next_idx)) = decode_next(dict_string) {
                    let value_print = value.clone();

                    // Continue from the next token
                    dict_string = &dict_string[next_idx..];

                    // Insert values in map
                    dict.insert(key.to_string(), value);

                    // Increment step
                    step += next_idx;
                    println!("end_idx: {}, key: {}, value: {:?}", step, key_print, value_print);
                } else {
                    break;
                }

                // Increment step
                step += next_idx;
            } else {
                break;
            }
        }

        return (serde_json::Value::Object(dict), step);
    }

    fn decode_list(encoded_string: &str) -> (serde_json::Value, usize) {
        println!("\nList char: {}", encoded_string);

        let mut array: Vec<Value> = Vec::new();

        // Starting from index '1'
        let mut list_string: &str = encoded_string.clone();
        list_string = &list_string[1..];
        let mut step: usize = 2; // + beginning and end characters for dict pattern

        println!("List string: {:?}", list_string);

        loop {
            // Decode the next single value (str or int)
            if let Some((value, next_idx)) = decode_next(list_string) {
                // Continue from the next token
                list_string = &list_string[next_idx..];
                
                let value_print = value.clone();
                array.push(value);
                step += next_idx;
                
                println!("end_idx: {}, value: {:?}", step, value_print);
            } else {
                break;
            }
        }

        return (serde_json::Value::Array(array), step);
    }

    fn decode_string(encoded_string: &str) -> (serde_json::Value, usize) {
        let colon_index = encoded_string.find(':').unwrap();
        let number_string = &encoded_string[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let end_idx: usize = colon_index + 1 + number as usize;
        let string = &encoded_string[colon_index + 1..end_idx];
        return (serde_json::Value::String(string.to_string()), end_idx);
    }

    fn decode_integer(encoded_string: &str) -> (serde_json::Value, usize) {
        let end_idx: usize = encoded_string.find('e').unwrap();
        let number_string: &str = &encoded_string[1..end_idx];
        let number: i64 = number_string.parse::<i64>().unwrap();
        let json_number: Number = Number::from(number);
        return (serde_json::Value::Number(json_number), end_idx+1);
    }
}