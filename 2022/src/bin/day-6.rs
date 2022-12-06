const INPUT: &str = include_str!("day-6.txt");

aoc_2022::aoc! {
    struct Day6<'a> {
        input: &'a [u8],
    }

    self(input) {
        Ok(Self { input: input.trim().as_bytes() })
    }

    part1 usize {
        for (i, w) in self.input.windows(4).enumerate() {
            if w[0] != w[1] && w[0] != w[2] && w[0] != w[3] &&
                w[1] != w[2] && w[1] != w[3] &&
                w[2] != w[3]
            {
                return Ok(i + 4);
            }
        }
        Err("not found".into())
    }

    part2 usize {
        'find: for (i, w) in self.input.windows(14).enumerate() {
            for j in 1..14 {
                if w[j..].contains(&w[j-1]) {
                    continue 'find;
                }
            }
            return Ok(i + 14);
        }
        Err("not found".into())
    }

    input = INPUT;
    test day6_ex1("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19);
    test day6_ex2("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23);
    test day6_ex3("nppdvjthqldpwncqszvftbrmjlhg", 6, 23);
    test day6_ex4("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29);
    test day6_ex5("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26);
    test day6(INPUT, 1658, 2260);
}
