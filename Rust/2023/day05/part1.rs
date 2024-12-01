pub fn run() -> u64 {
    let input = include_str!("../../inputs/input_2023_5.txt");
    lowest_location(input.lines())
}

fn lowest_location<'a, I: Iterator<Item = &'a str>>(mut lines: I) -> u64 {
    let seeds = parse_seeds(lines.next().unwrap());
    let mut lines = lines.skip(2);

    let set = parse_map_set(&mut lines);

    seeds.iter().map(|s| calc_location(&set, *s)).min().unwrap()
}

#[derive(Debug)]
pub(super) struct MapSet {
    pub seed_to_soil: Vec<MapRange>,
    pub soil_to_fertilizer: Vec<MapRange>,
    pub fertilizer_to_water: Vec<MapRange>,
    pub water_to_light: Vec<MapRange>,
    pub light_to_temperature: Vec<MapRange>,
    pub temperature_to_humidity: Vec<MapRange>,
    pub humidity_to_location: Vec<MapRange>,
}

#[derive(Debug)]
pub(super) struct MapRange {
    pub dest: u64,
    pub src: u64,
    pub len: u64,
}

impl MapRange {
    pub fn map(&self, value: u64) -> Option<u64> {
        if (self.src..self.src + self.len).contains(&value) {
            Some(self.dest + (value - self.src))
        } else {
            None
        }
    }
}

pub(super) fn calc_location(set: &MapSet, mut value: u64) -> u64 {
    value = map(&set.seed_to_soil, value);
    value = map(&set.soil_to_fertilizer, value);
    value = map(&set.fertilizer_to_water, value);
    value = map(&set.water_to_light, value);
    value = map(&set.light_to_temperature, value);
    value = map(&set.temperature_to_humidity, value);
    map(&set.humidity_to_location, value)
}

fn map(maps: &[MapRange], value: u64) -> u64 {
    for m in maps {
        if let Some(v) = m.map(value) {
            return v;
        }
    }

    value
}

pub(super) fn parse_map_set<'a, I: Iterator<Item = &'a str>>(mut lines: &mut I) -> MapSet {
    let seed_to_soil = parse_map(&mut lines);
    let mut lines = lines.skip(1);

    let soil_to_fertilizer = parse_map(&mut lines);
    let mut lines = lines.skip(1);

    let fertilizer_to_water = parse_map(&mut lines);
    let mut lines = lines.skip(1);

    let water_to_light = parse_map(&mut lines);
    let mut lines = lines.skip(1);

    let light_to_temperature = parse_map(&mut lines);
    let mut lines = lines.skip(1);

    let temperature_to_humidity = parse_map(&mut lines);
    let mut lines = lines.skip(1);

    let humidity_to_location = parse_map(&mut lines);

    MapSet {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

pub(super) fn parse_seeds(line: &str) -> Vec<u64> {
    line[7..]
        .trim()
        .split(' ')
        .map(|l| l.parse().unwrap())
        .collect()
}

fn parse_map<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Vec<MapRange> {
    let mut map = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            break;
        }

        let numbers = line
            .split(' ')
            .take(3)
            .map(|p| p.parse().unwrap())
            .collect::<Vec<u64>>();

        if numbers.len() != 3 {
            continue;
        }

        map.push(MapRange {
            dest: numbers[0],
            src: numbers[1],
            len: numbers[2],
        });
    }

    map.sort_by(|m1, m2| m1.src.cmp(&m2.src));
    map
}
