pub(crate) fn eval_file(file: &str) -> u64 {
    let convert: Convert = file.into();
    convert.to_location()
}

pub(crate) fn eval_file_2(file: &str) -> u64 {
    let convert: Convert = file.into();
    convert.to_location_range()
}

#[derive(Debug)]
struct Range {
    dest: u64,
    origin: u64,
    range: u64,
}
impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');
        let dest = split
            .next()
            .expect("Should have 1 nbr")
            .parse::<u64>()
            .expect("Is a nbr");
        let origin = split
            .next()
            .expect("Should have 2 nbr")
            .parse::<u64>()
            .expect("Is a nbr");
        let range = split
            .next()
            .expect("Should have 3 nbr")
            .parse::<u64>()
            .expect("Is a nbr");
        Self {
            dest,
            origin,
            range,
        }
    }
}

impl Range {
    fn transform(&self, value: u64) -> Option<u64> {
        if self.origin <= value && value < self.origin + self.range {
            Some(self.dest + value - self.origin)
        } else {
            None
        }
    }
}

fn convert_from_ranges(ranges: &Vec<Range>, value: u64) -> u64 {
    ranges
        .iter()
        .find_map(|range| range.transform(value))
        .unwrap_or(value)
}

#[derive(Debug)]
struct Convert {
    seeds: Vec<u64>,
    to_soil: Vec<Range>,
    to_fertilizer: Vec<Range>,
    to_water: Vec<Range>,
    to_light: Vec<Range>,
    to_temperature: Vec<Range>,
    to_humidity: Vec<Range>,
    to_location: Vec<Range>,
}

impl Convert {
    fn to_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| {
                let soil = convert_from_ranges(&self.to_soil, *seed);
                let fertilizer = convert_from_ranges(&self.to_fertilizer, soil);
                let water = convert_from_ranges(&self.to_water, fertilizer);
                let light = convert_from_ranges(&self.to_light, water);
                let temperature = convert_from_ranges(&self.to_temperature, light);
                let humidity = convert_from_ranges(&self.to_humidity, temperature);
                let location = convert_from_ranges(&self.to_location, humidity);
                location
            })
            .min()
            .expect("has a min")
    }

    fn to_location_range(&self) -> u64 {
        self.seeds
            .chunks(2)
            .flat_map(|w| {
                (w[0]..(w[0] + w[1])).map(|seed| {
                    let soil = convert_from_ranges(&self.to_soil, seed);
                    let fertilizer = convert_from_ranges(&self.to_fertilizer, soil);
                    let water = convert_from_ranges(&self.to_water, fertilizer);
                    let light = convert_from_ranges(&self.to_light, water);
                    let temperature = convert_from_ranges(&self.to_temperature, light);
                    let humidity = convert_from_ranges(&self.to_humidity, temperature);
                    let location = convert_from_ranges(&self.to_location, humidity);
                    location
                })
            })
            .min()
            .expect("has a min")
    }
}

fn transform_lines(block: &str) -> Vec<Range> {
    let mut lines = block.lines();
    lines.next();
    lines.map(|line| line.into()).collect::<Vec<Range>>()
}

impl From<&str> for Convert {
    fn from(value: &str) -> Self {
        let value = value.replace('\r', "");
        let mut parts = value.split("\n\n");
        let mut seeds = parts.next().expect("seeds present").split(": ");
        seeds.next();
        let seeds = seeds
            .next()
            .expect("Seeds present")
            .split(' ')
            .map(|v| v.parse::<u64>().expect("Seed is nbr"))
            .collect::<Vec<_>>();
        let to_soil = transform_lines(parts.next().expect("to_soil present"));

        let to_fertilizer = transform_lines(parts.next().expect("to_fertilizer present"));
        let to_water = transform_lines(parts.next().expect("to_water present"));
        let to_light = transform_lines(parts.next().expect("to_light present"));
        let to_temperature = transform_lines(parts.next().expect("to_temperature present"));
        let to_humidity = transform_lines(parts.next().expect("to_humidity present"));
        let to_location = transform_lines(parts.next().expect("to_location present"));

        Self {
            seeds,
            to_soil,
            to_fertilizer,
            to_water,
            to_light,
            to_temperature,
            to_humidity,
            to_location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{eval_file, eval_file_2};

    fn data() -> &'static str {
        r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#
    }
    #[test]
    fn test_0() {
        assert_eq!(35, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(46, eval_file_2(data()));
    }
}
