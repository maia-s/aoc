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
    println!("part 2: {}", part_2(&passports));
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

fn part_2(passports: &[Passport]) -> usize {
    let mut valid = 0;
    for passport in passports.iter() {
        // :=
        if let Some(birth_year) = passport.birth_year {
            if birth_year < 1920 || birth_year > 2002 {
                continue;
            }
        } else {
            continue;
        }
        if let Some(issue_year) = passport.issue_year {
            if issue_year < 2010 || issue_year > 2020 {
                continue;
            }
        } else {
            continue;
        }
        if let Some(expiration_year) = passport.expiration_year {
            if expiration_year < 2020 || expiration_year > 2030 {
                continue;
            }
        } else {
            continue;
        }
        if let Some(height) = &passport.height {
            if height.ends_with("cm") {
                if let Ok(height) = height[..(height.len() - 2)].parse::<u32>() {
                    if height < 150 || height > 193 {
                        continue;
                    }
                } else {
                    continue;
                }
            } else if height.ends_with("in") {
                if let Ok(height) = height[..(height.len() - 2)].parse::<u32>() {
                    if height < 59 || height > 76 {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            continue;
        }
        if let Some(hair_color) = &passport.hair_color {
            if hair_color.len() != 7 {
                continue;
            }
            let mut it = hair_color.chars();
            if it.next().unwrap() != '#' {
                continue;
            }
            let mut ok = true;
            while let Some(digit) = it.next() {
                if !((digit >= '0' && digit <= '9') || (digit >= 'a' && digit <= 'f')) {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }
        } else {
            continue;
        }
        if let Some(eye_color) = &passport.eye_color {
            let eye_color: &str = &eye_color;
            match eye_color {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                _ => continue,
            }
        } else {
            continue;
        }
        if let Some(passport_id) = &passport.passport_id {
            if passport_id.len() != 9 || !passport_id.parse::<u32>().is_ok() {
                continue;
            }
        } else {
            continue;
        }     
        valid += 1;
    }
    valid
}
