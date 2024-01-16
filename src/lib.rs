use std::fmt;

use chrono::NaiveDate;

const LENGTH: usize = 18;

const COEFFICIENT: [usize; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];

const CHECK: [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "男"),
            Gender::Female => write!(f, "女"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Length(usize),
    InvalidDate(String),
    NotNumber(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Length(len) => write!(f, "长度必须是18位，当前为{}位", len),
            Error::InvalidDate(curr) => write!(f, "无效的生日日期 '{}'", curr),
            Error::NotNumber(c) => write!(f, "'{}'不是有效的数字", c),
        }
    }
}

pub struct ChinaId(pub(crate) String);

impl fmt::Display for ChinaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl ChinaId {
    pub fn new(raw: &str) -> ChinaId {
        ChinaId(raw.to_uppercase())
    }

    pub fn valid(&self) -> Result<()> {
        // must be 18
        if self.0.len() != LENGTH {
            return Err(Error::Length(self.0.len()));
        }

        let chars = self.0.chars();
        let mut sum = 0_usize;

        for (i, c) in chars.enumerate() {
            // index: 18
            if i == LENGTH - 1 {
                let y = (sum % 11) as usize;
                if CHECK[y] == c {
                    //  ALL Good
                    break;
                }
            }

            // index: 0-17
            // must be number
            match c.to_string().parse::<usize>() {
                Ok(c) => {
                    sum += c * COEFFICIENT[i];
                }
                Err(_) => return Err(Error::NotNumber(c)),
            }
        }

        // birthday must be valid
        if let Err(err) = self.birthday() {
            return Err(err);
        }

        Ok(())
    }

    pub fn adcode(&self) -> &str {
        &self.0[..6]
    }

    pub fn birthday(&self) -> Result<NaiveDate> {
        // must be 18
        if self.0.len() != LENGTH {
            return Err(Error::Length(self.0.len()));
        }

        let date = &self.0[6..14];
        match NaiveDate::parse_from_str(date, "%Y%m%d") {
            Ok(date) => Ok(date),
            Err(_) => Err(Error::InvalidDate(date.to_string())),
        }
    }

    pub fn gender(&self) -> Gender {
        if let Some(c) = self.0.chars().nth(16) {
            if let Ok(u) = c.to_string().parse::<usize>() {
                return match u % 2 {
                    0 => Gender::Female,
                    _ => Gender::Male,
                };
            }
        }

        Gender::Male
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_parse() {
        let id = ChinaId::new("43102220200101133x");

        assert_eq!(format!("{}", id), "43102220200101133X");
        assert!(id.valid().is_ok());
        assert_eq!(id.adcode(), "431022");
        let b = id.birthday();
        assert!(b.is_ok());
        assert_eq!(b.unwrap().to_string(), "2020-01-01");
        assert_eq!(id.gender(), Gender::Male);
    }
}
