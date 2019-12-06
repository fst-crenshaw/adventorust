pub mod lights {

    use regex::Regex;
    use std::fmt;

    const CG_INSTRUCTION: usize = 1;
    const CG_LOCATIONS: usize = 2;
    const CG_TOTAL: usize = 6;

    #[derive(Debug, PartialEq)]
    pub struct Location(usize, usize);

    pub enum BulbAction {
        TurnOn,
        TurnOff,
        Toggle,
    }

    #[derive(PartialEq, Copy, Clone)]
    pub enum BulbValue {
        On,
        Off,
    }

    impl fmt::Debug for BulbValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut d = f.debug_struct("BulbValue");
            let mut printable = String::new();
            match self {
                BulbValue::On => {
                    printable.push('*');
                    d.field("", &printable);
                    d.finish()
                }
                BulbValue::Off => {
                    printable.push('-');
                    d.field("", &printable);
                    d.finish()
                }
            }
        }
    }

    impl fmt::Display for BulbValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                BulbValue::On => write!(f, "{}", "*"),
                BulbValue::Off => write!(f, "{}", "-"),
            }
        }
    }

    impl std::ops::Not for BulbValue {
        type Output = BulbValue;

        fn not(self) -> Self::Output {
            match self {
                BulbValue::On => BulbValue::Off,
                BulbValue::Off => BulbValue::On,
            }
        }
    }

    pub struct LightGrid {
        pub grid: [[u32; 1000]; 1000],
    }

    impl LightGrid {
        pub fn _pretty_print(&self) {
            for strand in self.grid.iter() {
                for bulb in strand.iter() {
                    print!("{}", bulb);
                }
                println!("");
            }
        }
        pub fn number_on(&self) -> u32 {
            let mut count = 0;
            for strand in self.grid.iter() {
                for bulb in strand.iter() {
                    count = count + bulb;
                }
            }
            count
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Instruction {
        Invalid,
        TurnOn { start: Location, end: Location },
        TurnOff { start: Location, end: Location },
        Toggle { start: Location, end: Location },
    }

    pub fn iterate_on_subset(
        bulb_action: BulbAction,
        l: &mut LightGrid,
        start: &Location,
        end: &Location,
    ) {
        for strand in l.grid[start.0..(end.0 + 1)].iter_mut() {
            for light_bulb in strand[start.1..(end.1 + 1)].iter_mut() {
                match bulb_action {
                    BulbAction::TurnOn => *light_bulb = light_bulb.saturating_add(1), //BulbValue::On,
                    BulbAction::TurnOff => *light_bulb = light_bulb.saturating_sub(1), //BulbValue::Off,
                    BulbAction::Toggle => *light_bulb = light_bulb.saturating_add(2),
                }
            }
        }
    }

    /* Given an instruction and a light grid, execute the
     * instruction on the LightGrid by turning on, turning off
     * or toggling the light. */
    pub fn execute(i: &Instruction, l: &mut LightGrid) -> bool {
        match i {
            Instruction::TurnOn { start: s, end: e } => {
                iterate_on_subset(BulbAction::TurnOn, l, s, e);
            }
            Instruction::TurnOff { start: s, end: e } => {
                iterate_on_subset(BulbAction::TurnOff, l, s, e);
            }
            Instruction::Toggle { start: s, end: e } => {
                iterate_on_subset(BulbAction::Toggle, l, s, e);
            }
            _ => {
                return false;
            }
        }
        true
    }

    /* Given a string like `toggle 111,55 through 880,871`
     * return a tuple comprising the Instruction, the Start
     * Location and the End Location */
    pub fn parse(s: &str) -> Instruction {
        let re = Regex::new(
            r"^(toggle|turn on|turn off) (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})$",
        )
        .unwrap();

        if !re.is_match(&s) {
            return Instruction::Invalid;
        }

        let cap = re.captures(&s).unwrap();
        if cap.len() != CG_TOTAL {
            return Instruction::Invalid;
        }

        let mut coordinates = Vec::new();
        for i in CG_LOCATIONS..cap.len() {
            let orig = &cap[i];
            match orig.parse::<usize>() {
                Ok(coordinate) => coordinates.push(coordinate),
                Err(_) => return Instruction::Invalid,
            }
        }

        let start = Location(coordinates[0], coordinates[1]);
        let end = Location(coordinates[2], coordinates[3]);

        match &cap[CG_INSTRUCTION] {
            "toggle" => return Instruction::Toggle { start, end },
            "turn on" => return Instruction::TurnOn { start, end },
            "turn off" => return Instruction::TurnOff { start, end },
            _ => return Instruction::Invalid,
        }
    }

    // Tests
    fn test_parse(s: &str, expected: Instruction) -> bool {
        if parse(&s) == expected {
            return true;
        }
        false
    }

    pub fn test_lights() {
        assert!(test_parse(
            "definitely not an instruction",
            Instruction::Invalid
        ));
        assert!(test_parse(
            "toggle 111,55 through 880,871",
            Instruction::Toggle {
                start: Location(111, 55),
                end: Location(880, 871)
            }
        ));
        assert!(test_parse(
            "turn off 111,55 through 880,871",
            Instruction::TurnOff {
                start: Location(111, 55),
                end: Location(880, 871)
            }
        ));
    }
}
