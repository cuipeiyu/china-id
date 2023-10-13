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
use std::{fmt, time};

const ID_LENGTH: usize = 18;

static COEFFICIENT: &'static [u8; 17] = &[7,9,10,5,8,4,2,1,6,3,7,9,10,5,8,4,2];
static CHECK: &'static [u8; 11] = &[1, 0, 255, 9, 8, 7, 6, 5, 4, 3, 2];

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

// impl fmt::Debug for Gender {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Gender::Male => write!(f, "Length must be 18"),
//         }
//     }
// }

#[cfg(adcode)]
#[derive(Debug)]
pub enum AreaRank {
    Province,
    City,
    County,
}

#[cfg(chinese)]
impl fmt::Debug for AreaRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.longitude)
            .field(&self.latitude)
            .finish()
    }
}

#[cfg(adcode)]
#[derive(Debug)]
pub enum Adcode {
    安徽省 = (J::Province, 340000),
    滁州市 = (J::City, 341100),
    琅琊区 = (J::County, 341103),
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

struct ChinaId  {
    raw: String,
    // 0-17
    id17: Vec<u8>,
    // 18
    id18: u8,
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
        let (is_valid, id17, id18) = Self::_is_valid(id);

        if !is_valid {
            // let e = fmt::Error::default("ChinaId");
            return Err(Error::Length);
        }

        Ok(ChinaId {
            raw: id.to_string(),
            id17: id17,
            id18: id18,
        })
    }

    fn _is_valid(id: &str) -> (bool, Vec<u8>, u8) {
        let mut list = vec![0 as u8; 17];

        if id.len() == ID_LENGTH {
            let mut sum = 0;

            for (i, c) in id.chars().enumerate() {
                match c.to_string().parse::<u8>() {
                    Ok(c) => {
                        list[i] = c;
                        sum += c * COEFFICIENT[i];
                    },
                    Err(_) => break,
                }
            }

            let y = (sum % 11) as usize;
            CHECK[y];
        }

        (false, list, 0)
    }

    pub fn is_valid(id: &str) -> bool {
        let (is_valid, _, _) = Self::_is_valid(id);
        is_valid
    }

    pub fn len(&self) -> usize {
        ID_LENGTH
    }

    // pub fn birthday(&self) -> time::Time {
    //     time::Time::
    // }

    pub fn gender(&self) -> Gender {
        match self.id17[17] % 2 {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }

    pub fn province_name(&self) -> &str {
        ""
    }

    #[cfg(chinese)]
    pub fn province_name_zh(&self) -> &str {
        ""
    }

    pub fn must_valid(id: &str) {
        if !Self::is_valid(id) {
            panic!("{} is not valid", id);
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_empty() {
        assert_eq!(ChinaId::is_valid(""), false);

        let _ = ChinaId::new("").unwrap();
    }

    #[test]
    #[should_panic]
    fn ut_must_valid() {
        ChinaId::must_valid("")
    }
}
