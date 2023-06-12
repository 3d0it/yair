#[derive(Debug)]
pub struct Config {
    pub image_path: String,
    pub percentage: u32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let image_path = &args[1];
        let percentage = &args[2];

        let percentage = match percentage.parse::<u32>() {
            Ok(n) if n > 100 => {
                return Err("Percentage must be lesser then 100");
            }
            Ok(n) => n,
            Err(_) => return Err("Percentage must be a positive number"),
        };

        Ok(Config {
            image_path: image_path.to_string(),
            percentage,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_err;

    #[test]
    fn build_wrong_percentage() {
        let args = vec![String::from("bears.jpg"), String::from("xxx")];
        assert_err!(Config::build(&args));
    }

    #[test]
    fn build_negative_percentage() {
        let args = vec![String::from("bears.jpg"), String::from("-10")];
        assert_err!(Config::build(&args));
    }
}
