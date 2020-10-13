use std::fs;
use std::env;

#[allow(non_camel_case_types)] pub type uxsize = u64;
#[allow(non_camel_case_types)] pub type ixsize = i64;

fn main() {
    let mut args = env::args();
    args.next();
    let src = args.next().expect("bltn <in> <out> [add lines=false]");
    let dst = args.next().expect("bltn <in> <out> [add lines=false]");
	let add_lines = args.next().map(|x| x.parse::<bool>().expect("expected true or false")).unwrap_or(false);
    let input = fs::read(src).unwrap();

    let output = process(&input, add_lines);

    fs::write(dst, output).unwrap();

}

fn process(data: &[u8], add_lines: bool) -> Vec<u8> {
	let mut vec = Vec::with_capacity(data.len() * 2);
	let lines = data.split(|&x| x == b'\n');
	for (line_n, line) in lines.enumerate() {
		let mut args = line.split(|x| x.is_ascii_whitespace());
		if !line.is_empty() {
			if line[0] == b'%' {
				if add_lines {
					vec.extend_from_slice(&format!("; % LINE: {}\n", line_n).as_bytes());
				}
				vec.extend_from_slice(line);
				vec.push(b'\n');
			} else if line[0] == b';' {
				if add_lines {
					vec.extend_from_slice(&format!("; ; LINE: {}\n", line_n).as_bytes());
				}
				vec.extend_from_slice(line);
				vec.push(b'\n');
			} else if line[0] == b'!' {
				if add_lines {
					vec.extend_from_slice(&format!("; ! LINE: {}\n", line_n).as_bytes());
				}
				vec.extend_from_slice(&line[1..]);
				vec.push(b'\n');
			} else if line[0] == b'~' {
				if line.len() == 1 {
					vec.push(b'\n');
				} else {
					vec.extend_from_slice(&line[1..]);
				}
			} else {
				let src = std::str::from_utf8(args.next().expect(&format!("error on line {}: missing src", line_n))).unwrap();
				let dst = std::str::from_utf8(args.next().expect(&format!("error on line {}: missing dst", line_n))).unwrap();
				let jmp = std::str::from_utf8(args.next().expect(&format!("error on line {}: missing jmp", line_n))).unwrap();
				let (sl, sv) = lv(src);
				let (dl, dv) = lv(dst);
				let (jl, jv) = lv(jmp);
				vec.extend_from_slice(&if add_lines {
					format!("{} dq {}\n{} dq{}\n{} dq {}\n", sl, sv, dl, dv, jl, jv)
				} else {
					format!(";   LINE: {}\n{} dq {}\n{} dq{}\n{} dq {}\n", line_n, sl, sv, dl, dv, jl, jv)
				}.as_bytes());
			}
		}

	}
	vec
}

fn lv(src: &str) -> (&str, &str) {
	let mut s = src.split(|x| x == ':');
	let sl = s.next().unwrap();
	let sv = s.next();
	if let Some(sv) = sv {
		(sl, sv)
	} else {
		("", sl)
	}
}