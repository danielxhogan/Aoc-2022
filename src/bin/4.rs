type RangeParts = (u32, u32, u32, u32);

struct SectionRange {
    start: u32,
    end: u32,
}

struct Pair {
    first: SectionRange,
    second: SectionRange,
}

impl Pair {
    fn get_parts(input: &str) -> Result<RangeParts, color_eyre::Report> {
        let halves = input.split(",").collect::<Vec<&str>>();

        if halves.len() != 2 {
            return Err(color_eyre::eyre::eyre!("input must be a pair"));
        }

        let mut halves = halves.iter();
        let mut parts: Vec<u32> = Vec::new();

        while let Some(half) = halves.next() {
            let range = half.split("-").collect::<Vec<&str>>();

            if range.len() != 2 {
                return Err(color_eyre::eyre::eyre!("range must be a pair"));
            }

            let mut range = range.iter();

            while let Some(part) = range.next() {
                let part: Result<u32, _> = part.parse();

                match part {
                    Ok(number) => parts.push(number),
                    Err(err) => {
                        return Err(color_eyre::eyre::eyre!(
                            "failed to parse number - err: {err:?}"
                        ));
                    }
                }
            }
        }

        let parts: RangeParts = (parts[0], parts[1], parts[2], parts[3]);
        Ok(parts)
    }

    fn check_containing(self) -> u32 {
        if self.first.start >= self.second.start && self.first.end <= self.second.end
            || self.second.start >= self.first.start && self.second.end <= self.first.end
        {
            return 1;
        } else {
            return 0;
        }
    }

    fn check_overlapping(self) -> u32 {
        if self.first.end >= self.second.start && self.first.start <= self.second.end {
            return 1;
        } else {
            return 0;
        }
    }
}

impl TryFrom<&str> for Pair {
    type Error = color_eyre::Report;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let parts = Self::get_parts(&input)?;

        let first_section = SectionRange {
            start: parts.0,
            end: parts.1,
        };

        let second_section = SectionRange {
            start: parts.2,
            end: parts.3,
        };

        return Ok(Pair {
            first: first_section,
            second: second_section,
        });
    }
}

fn main() -> Result<(), color_eyre::Report> {
    let sum = include_str!("../input/4.txt")
        .lines()
        .map(|line| -> Result<_, color_eyre::Report> {
            let pair = Pair::try_from(line)?;
            Ok(pair.check_containing())
        })
        .sum::<color_eyre::Result<u32>>();

    println!("sum: {sum:?}");

    let sum = include_str!("../input/4.txt")
        .lines()
        .map(|line| -> Result<_, color_eyre::Report> {
            let pair = Pair::try_from(line)?;
            Ok(pair.check_overlapping())
        })
        .sum::<color_eyre::Result<u32>>();

    println!("sum: {sum:?}");
    Ok(())
}
