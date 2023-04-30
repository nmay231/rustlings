// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in the form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    use Command::*;

    // Semantically, is it better to use &str or String here? I thought String was converted to &str automatically, but that doesn't always appear to be the case
    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        let mut output = vec![];
        for (string, command) in input.iter() {
            output.push(match *command {
                Uppercase => string.to_uppercase(),
                Trim => string.trim().to_string(),
                Append(times) => {
                    let repeated = "bar".repeat(times);
                    format!("{string}{repeated}")
                }
            })
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello", Command::Uppercase),
            (" all roads lead to rome! ", Command::Trim),
            ("foo", Command::Append(1)),
            ("bar", Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }

    fn to_string(input: &str) -> String {
        input.to_string()
    }

    #[test]
    fn is_string_coerced_automatically() {
        let output = to_string("test1");
        // Apparently not...
        // let output = to_string("test1".to_string());
    }
}
