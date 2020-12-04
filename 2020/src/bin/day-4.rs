const INPUT: &str = include_str!("day-4.input");

#[derive(Default)]
struct Passport {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

fn main() {
    let mut passports = vec![];
    let mut passport = Passport::default();
    for line in INPUT.lines() {
        let line = line.trim();
        if line.is_empty() {
            passports.push(passport);
            passport = Passport::default();
        } else {
            for field in line.split_ascii_whitespace() {
                let mut field = field.splitn(2, ':');
                let name = field.next().expect("field name");
                let value = field.next().expect("field value");
                match name {
                    "byr" => passport.birth_year = Some(value.parse().expect("byr")),
                    "iyr" => passport.issue_year = Some(value.parse().expect("iyr")),
                    "eyr" => passport.expiration_year = Some(value.parse().expect("eyr")),
                    "hgt" => passport.height = Some(value.to_string()),
                    "hcl" => passport.hair_color = Some(value.to_string()),
                    "ecl" => passport.eye_color = Some(value.to_string()),
                    "pid" => passport.passport_id = Some(value.to_string()),
                    "cid" => passport.country_id = Some(value.to_string()),
                    _ => (),
                }
            }
        }
    }

    println!("part 1: {}", part_1(&passports));
}

fn part_1(passports: &[Passport]) -> usize {
    let mut valid = 0;
    for passport in passports.iter() {
        valid += (passport.birth_year.is_some()
            && passport.issue_year.is_some()
            && passport.expiration_year.is_some()
            && passport.height.is_some()
            && passport.hair_color.is_some()
            && passport.eye_color.is_some()
            && passport.passport_id.is_some()) as usize;
    }
    valid
}
