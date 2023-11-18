use std::fs;
use std::env;

#[allow(non_camel_case_types)] pub type uxsize = u64;
#[allow(non_camel_case_types)] pub type ixsize = i64;

fn main() {
    let mut args = env::args();
    args.next();
    let src = args.next().expect("bltn <in> <out> [add lines=false] [file identifier=<in>]");
    let dst = args.next().expect("bltn <in> <out> [add lines=false] [file identifier=<in>]");
	let add_lines = args.next().map(|x| x.parse::<bool>().expect("expected true or false")).unwrap_or(false);
	let file_ident = args.next().unwrap_or_else(|| {
		src.replace("/", "_SLASH_").replace(".", "_DOT_")
	});
    let input = fs::read(&src).unwrap();

    let output = process(&input, &file_ident, &src, add_lines);

    fs::write(dst, output).unwrap();

}

fn process(data: &[u8], file_ident: &str, file_path: &str, add_lines: bool) -> Vec<u8> {
	let mut vec = Vec::with_capacity(data.len() * 4);
	vec.extend_from_slice(&format!(
		concat!(
			"; === AUTOGENERATED BY bltn === ;\n",
			"%ifndef __BLTN_FILE_{}\n",
			"%define __BLTN_FILE_{}\n",
			"%ifndef BLTN_D\n",
			"%define BLTN_D dq\n",
			"%define BLTN_WORD_SIZE 8\n",
			"%endif\n"
		), file_ident, file_ident
	).as_bytes());
	let lines = data.split(|&x| x == b'\n');
	for (mut line_n, line) in lines.enumerate() {
		line_n += 1;
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
				let mut args = line.split(|x| x.is_ascii_whitespace());
				let src = std::str::from_utf8(args.next().expect(&format!("error on line {}: missing src", line_n))).unwrap();
				let dst = std::str::from_utf8(args.next().expect(&format!("error on line {}: missing dst", line_n))).unwrap();
				let jmp = args.next().map(|x| std::str::from_utf8(x).unwrap()).unwrap_or("($-$$)+BLTN_WORD_SIZE");
				let (sl, sv) = lv(src);
				let (dl, dv) = lv(dst);
				let (jl, jv) = lv(jmp);
				vec.extend_from_slice(&if add_lines {
					format!(";   LINE: {}\n{} BLTN_D {}\n{} BLTN_D {}\n{} BLTN_D {}\n", line_n, sl, sv, dl, dv, jl, jv)
				} else {
					format!("{} BLTN_D {}\n{} BLTN_D {}\n{} BLTN_D {}\n", sl, sv, dl, dv, jl, jv)
				}.as_bytes());
			}
		}

	}
	vec.extend_from_slice(&format!("\n%else\n%error File {} included multiple times. Current position: %[_POS]\n%endif\n", file_path).as_bytes());
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
