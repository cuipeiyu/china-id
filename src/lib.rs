/// 行政区划代码的含义
///
/// 《县以下行政区划代码编制规则》是《中华人民共和国行政区划代码》的补充和拓延，它规定了表示乡、镇（街道/// 办事处）一级行政区划的三位代码。
///
/// 代码从左至右的含义是：
///
/// 第一、二位表示省（自治区、直辖市、特别行政区）。
///
/// 第三、四位表示市（地区、自治州、盟及国家直辖市所属市辖区和县的汇总码）。其中，01-20，51-70表示省直辖市；21-50表示地区（自治州、盟）。
///
/// 第五、六位表示县（市辖区、县级市、旗）。01-20表示市辖区或地区（自治州、盟）辖县级市；21-70表示县（旗）；81-99表示省直辖县级市；71-80表示工业园区或者经济开发区。
///
/// 第七至九位表示乡、镇（街道办事处）。其中000-099表示街道办事处，100-199表示镇，200-299表示乡，400-479表示林场、牧场、科技城、园区，480-499表示林业管理局，500-579表示农场、团，580-599表示畜牧场。
///
/// 为了更详细地反映乡镇以下区划情况，国家统计局补充了三位表示居委会、村委会的代码。

use std::fmt;

use chrono::NaiveDate;

const ID_LENGTH: usize = 18;

static COEFFICIENT: &'static [u8; 17] = &[7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
static CHECK: &'static [char; 11] = &['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

#[derive(Debug, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

pub enum Error {
    Length,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Length => write!(f, "Length must be 18"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Length => write!(f, "Length must be 18"),
        }
    }
}

struct ChinaId {
    raw: String,
    birthday: NaiveDate,
    // 0-17
    index_0_17: Vec<u8>,
    // 18
    index_18: char,
}

impl fmt::Display for ChinaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.raw)
    }
}

impl fmt::Debug for ChinaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.raw)
    }
}

impl ChinaId {
    pub fn new(id: &str) -> Result<ChinaId, Error> {
        let i = Vec::with_capacity(17);

        let mut res = ChinaId {
            raw: id.to_uppercase(),
            birthday: NaiveDate::default(),
            index_0_17: i,
            index_18: '\0',
        };

        let chars = res.raw.chars();

        if chars.clone().count() != ID_LENGTH {
            return Err(Error::Length);
        }

        let mut sum = 0;

        for (i, c) in chars.enumerate() {
            // index: 18
            if i == ID_LENGTH - 1 {
                let y = (sum % 11) as usize;
                if CHECK[y] == c {
                    //  ALL Good
                    return Ok(res);
                }
            }

            if i == 6 {
                match NaiveDate::parse_from_str(&res.raw.clone()[6..14], "%Y%m%d") {
                    Ok(b) => res.birthday = b,
                    Err(_) => return Err(Error::Length), // TODO 错误 日期解析错误
                }
            }

            // index: 0-17
            // must be number
            match c.to_string().parse::<u8>() {
                Ok(c) => {
                    res.index_0_17.push(c);
                    sum += c * COEFFICIENT[i];
                }
                Err(_) => return Err(Error::Length), // TODO 错误 不能为非数字
            }
        }

        return Err(Error::Length);
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn adcode(&self) -> &str {
        &self.raw[..6]
    }
    pub fn birthday(&self) -> NaiveDate {
        self.birthday
    }

    pub fn gender(&self) -> Gender {
        match self.index_0_17[16] % 2 {
            0 => Gender::Female,
            _ => Gender::Male,
        }
    }

    pub fn must_valid(res: Result<ChinaId, Error>) -> ChinaId {
        if let Err(e) = res {
            panic!("not valid {}", e);
        }

        res.ok().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn ut_empty() {
        let _ = ChinaId::new("").unwrap();
    }

    #[test]
    fn ut_parse() {
        let id = ChinaId::new("43102220200101133X").unwrap();

        assert_eq!(id.adcode(), "431022");
        assert_eq!(
            id.birthday(),
            NaiveDate::parse_from_str("20200101", "%Y%m%d").unwrap()
        );
        assert_eq!(id.gender(), Gender::Male);
    }
}
