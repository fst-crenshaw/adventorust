// Returns relative move for x, y
fn compute_move(s: &str, start_x: i32, start_y: i32) -> (i32, i32) {
	//println!("compute_move: s={}, x={}, y={}", s, start_x, start_y);
	let mut ex = start_x;
	let mut why = start_y;

	for c in s.chars() {
		let (x, y) = to_tuple(c);
		ex += x;
		why += y;

		ex = match ex {
			ex if ex > 1 => 1,
			ex if ex < -1 => -1,
			_ => ex
		};
		why = match why {
			why if why > 1 => 1,
			why if why < -1 => -1,
			_ => why
		};
	}

	(ex, why)
}

fn to_tuple(c: char) -> (i32, i32) {
	match c {
    	'U' => (0, 1),
    	'D' => (0, -1),
    	'L' => (-1, 0),
    	'R' => (1, 0),
		_ => panic!("invalid direction: {}", c),
	}
}

fn to_num(x: i32, y: i32) -> i32 {
	match (x, y) {
    	(-1, 1) => 1,
    	(0, 1) => 2,
    	(1, 1) => 3,
    	(-1, 0) => 4,
    	(0, 0) => 5,
    	(1, 0) => 6,
    	(-1, -1) => 7,
    	(0, -1) => 8,
    	(1, -1) => 9,
		_ => panic!("invalid tuple: x={}, y={}", x, y)
	}
}

fn main() {
	let ex1 = ["ULL", "RRDDD", "LURDL", "UUUUD"];
	let _numpad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

	println!("str=U, x=0, y=0, output={:?}", compute_move("U", 0, 0));
	println!("str=RU, x=0, y=0, output={:?}", compute_move("RU", 0, 0));
	println!("str=RULD, x=0, y=0, output={:?}", compute_move("RULD", 0, 0));

	let mut x = 0;
	let mut y = 0;
	for instr in &ex1 {
		let (newx, newy) = compute_move(instr, x, y);
		x = newx;
		y = newy;
		println!("digit={}", to_num(x, y));
	}
}
