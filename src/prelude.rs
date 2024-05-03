// 2024-05-03 falsefox
// GPL 3.0 Licensed

/// The struct template that contains the users's information.
pub struct Info {
    pub name: String,
    pub pronouns: Vec<String>,
    pub email: String,
    pub github: String,
    pub website: String,
}

// The builder for the struct.
fn builder() -> Info {
    let falsefox_info: Info = Info {
        name: String::from("Fox"),
        pronouns: vec![String::from("she/her")],
        email: String::from("me@falsefox.dev"),
        github: String::from("https://github.com/false-fox"),
        website: String::from("https://falsefox.dev"),
    };
    falsefox_info
}

/// Grabs all info and returns a struct.
pub fn get_info() -> Info {
    builder()
}

/// Returns just the email property.
pub fn get_email() -> String {
    builder().email
}

/// Returns just the name property.
pub fn get_name() -> String {
    builder().name
}

/// Returns just the github property.
pub fn get_github() -> String {
    builder().github
}

/// Returns just the website property.
pub fn get_website() -> String {
    builder().website
}

/// Returns just the pronouns property.
pub fn get_pronouns() -> Vec<String> {
    builder().pronouns
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_functions() {
        assert_eq!(builder().website, get_website());
        assert_eq!(builder().github, get_github());
        assert_eq!(builder().pronouns, get_pronouns());
        assert_eq!(builder().name, get_name());
        assert_eq!(builder().email, get_email());
 
    }
}