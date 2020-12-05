use {itertools::Itertools, regex::Regex};

lazy_static::lazy_static! {
    static ref HGT_REGEX: Regex = Regex::new("(?P<height>[0-9]{2,3})(?P<unit>in|cm)").unwrap();
    static ref HCL_REGEX: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
}

#[derive(Debug, Default)]
pub struct PartialPassport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl PartialPassport {
    pub fn validate_line_lax(line: &str) -> bool {
        let mut passport = Self::default();

        for field in line.split(|c| c == ' ' || c == '\n') {
            match field.split(':').collect_tuple().unwrap_or_default() {
                ("byr", _) => passport.byr = true,
                ("iyr", _) => passport.iyr = true,
                ("eyr", _) => passport.eyr = true,
                ("hgt", _) => passport.hgt = true,
                ("hcl", _) => passport.hcl = true,
                ("ecl", _) => passport.ecl = true,
                ("pid", _) => passport.pid = true,
                ("cid", _) => passport.cid = true,
                ("", _) => (),
                _ => unreachable!(),
            }
        }

        passport.valid()
    }

    pub fn validate_line_strict(line: &str) -> bool {
        let mut passport = Self::default();

        for field in line.split(|c| c == ' ' || c == '\n') {
            match field.split(':').collect_tuple().unwrap_or_default() {
                ("byr", val) => {
                    let val: u32 = val.parse().unwrap();
                    if 1920 <= val && val <= 2002 {
                        passport.byr = true;
                    } else {
                        return false;
                    }
                }
                ("iyr", val) => {
                    let val: u32 = val.parse().unwrap();
                    if 2010 <= val && val <= 2020 {
                        passport.iyr = true;
                    } else {
                        return false;
                    }
                }
                ("eyr", val) => {
                    let val: u32 = val.parse().unwrap();
                    if 2020 <= val && val <= 2030 {
                        passport.eyr = true;
                    } else {
                        return false;
                    }
                }
                ("hgt", val) => {
                    passport.hgt = if let Some((height, unit)) = HGT_REGEX
                        .captures(val)
                        .and_then(|caps| Some((caps.name("height")?, caps.name("unit")?)))
                    {
                        let height = height.as_str().parse::<u32>().unwrap();
                        match unit.as_str() {
                            "in" => 59 <= height && height <= 76,
                            "cm" => 150 <= height && height <= 193,
                            _ => return false,
                        }
                    } else {
                        return false;
                    };

                    if !passport.hgt {
                        return false;
                    }
                }
                ("hcl", val) => {
                    if HCL_REGEX.is_match(val) {
                        passport.hcl = true
                    } else {
                        return false;
                    }
                }
                ("ecl", val) => {
                    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .iter()
                        .any(|s| val == *s)
                    {
                        passport.ecl = true
                    } else {
                        return false;
                    }
                }
                ("pid", val) => {
                    if val.len() == 9 && val.chars().all(|c| c.is_digit(10)) {
                        passport.pid = true
                    } else {
                        return false;
                    }
                }
                ("cid", _) => passport.cid = true,
                ("", _) => (),
                _ => unreachable!(),
            }
        }

        passport.valid()
    }

    pub fn valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|line| PartialPassport::validate_line_lax(line))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|line| PartialPassport::validate_line_strict(line))
        .count()
}
