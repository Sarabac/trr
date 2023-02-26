use rand::distributions::Alphanumeric;
use rand::prelude::*;

#[derive(Debug)]
pub struct FileNameIter {
    pub size_range: (usize, usize),
    pub extensions: Vec<String>,
}

impl Default for FileNameIter {
    fn default() -> Self {
        Self {
            size_range: (2, 5),
            extensions: vec!["png".into(), "py".into(), "docx".into(), "sql".into()],
        }
    }
}

impl Distribution<String> for FileNameIter {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let size = {
            let (a, b) = self.size_range;
            if a < b {
                rng.gen_range(a..b)
            } else if a > b {
                rng.gen_range(b..a)
            } else {
                a
            }
        };

        let name: String = rng
            .sample_iter(&Alphanumeric)
            .take(size)
            .map(char::from)
            .collect();

        let extension = self
            .extensions
            .iter()
            .choose(rng)
            .map(String::from)
            .filter(|txt| !txt.is_empty());

        match extension {
            Some(ext) => format!("{name}.{ext}"),
            None => name,
        }
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use super::*;

    #[test]
    fn valid_names() {
        let rng = rand::thread_rng();
        let sampler = FileNameIter::default();
        let matcher = Regex::new(r"\.[a-zA-Z0-9]*$").unwrap();
        let test_filename: Vec<String> = sampler.sample_iter(rng).take(10).collect();
        dbg!(&test_filename);

        test_filename
            .iter()
            .for_each(|filename| assert!(matcher.is_match(filename)));
    }
}
