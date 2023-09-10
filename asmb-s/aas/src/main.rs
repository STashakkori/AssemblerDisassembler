use std::process::Command;
use std::env;
use std::io::{BufWriter, Write, Read};
use std::path::Path;
use std::fs::File;
use std::fs;
use std::process::Stdio;
fn main() {
	
	// Collect args from command line, check length "asm ppc32 "addi 3,3,48" len == 3"
	let args: Vec<String> = env::args().collect();
	if args.len() > 3 || args.len() < 3 {
			println!("  Usage: asm <arch type> 'instruction' ");
			return; 
	}
	let isa = args[1].trim().to_string();
	let instr = args[2].trim().to_string();
	
	// get file will take the instruction and create and write it to a file: tmp.s
	get_file(instr);
	
	// by this point we should have stored args[2] inside tmp.s
	let path_string = String::from("ext/core/tmp.s");
	
	
	////////////////////Safe Directives check start

	let mut sfd_file  = match File::open(&path_string) {
		Ok(input) => input,
		Err(err) => {
			println!("*Failed to read safe directives* Error : {}", err);
			return;
		}
	};
	
	let mut buffer = String::new();
	match sfd_file.read_to_string(&mut buffer) {
		Ok(input) => input,
		Err(err) => {
			println!("Failed to parse asm.txt, Error : {}", err);
			return;
		}
	};
	
	let inst: Vec::<&str> = buffer.split("\n").collect();
	if !check_safe_dir(inst) {
		return;
	}
	
	//////////////////Safe Directives check finish
	
	let path_check = Path::new("ext/core/tmp.o");
	// we now carry on as normal in asmb and feed the rest to our program
	let ulti_path = path_string.clone();
	match isa.trim() {
		"x86" => {
			asm_x86(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck();
			parsedumptruck(ulti_path);
		},
		"x64" => {
			asm_x64(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck();
			parsedumptruck(ulti_path);
		},
		"ppc32" => {
			asm_ppc(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_ppc();
			parsedumptruck(ulti_path);
		},
		"ppc64" => {
			asm_ppc64(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_ppc64();
			parsedumptruck(ulti_path);
		},
		"mips32" => {
			asm_mips(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_mips();
			parsedumptruck(ulti_path);
		},
		"mips64" => {
			asm_mips64(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_mips64();
			parsedumptruck(ulti_path);
		},
		"arm32" => {
			asm_arm(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_arm();
			parsedumptruck(ulti_path);
		},
		"arm64" => {
			asm_arm64(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_arm64();
			parsedumptruck(ulti_path);
		},
		"sparc32" => {
			asm_sparc32(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_sparc();
			parsedumptruck(ulti_path);
		},
		"sparc64" => {
			asm_sparc64(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_sparc();
			parsedumptruck(ulti_path);
		},
		"avr" => {
			asm_avr(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_avr();
			parsedumptruck(ulti_path);
		},
		"riscv32" => {
			asm_riscv32(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_riscv32();
			parsedumptruck(ulti_path);
		},	
		"riscv64" => {
			asm_riscv64(ulti_path.clone());
			if !path_check.exists() {println!("invalid instruction.");return;}
			objdumptruck_riscv64();
			parsedumptruck(ulti_path);
		},		
		_ => {
			println!("Invalid Architecture.");
			return;
		}
	};
	
	// here we will delete the unnecessary file that we used during our operations
	
	cleanup("ext/core/tmp.s".to_string(), "ext/core/tmp.o".to_string());
	cleanup("ext/core/asm.txt".to_string(), "ext/core/tmp-out.txt".to_string());
	
}

fn cleanup(input_file1: String, input_file2: String) {
	
	match fs::remove_file(input_file1) {
		Ok(x) => x,
		Err(..) => { 
			return;
		}
	};
	
	match fs::remove_file(input_file2) {
		Ok(x) => x,
		Err(..) => { 
			return;
		}
	};	
}

// support function for check_safe_dir
fn check_dot_cmd(line: &str) -> &str {
	let mut s = 0;
	let mut e = 0;
	let mut start_flag = false;
	for (pos,ele) in line.chars().enumerate() {
		if ele == '.' {
			s = pos;
			start_flag = true;
			//print!("{}", s);
			//print!("[start]");
			
		}
		if ele == ' ' && start_flag != false {
			e = pos;
			//print!("[end]");
			break;
		}
	}
	if s == 0 && e == 0 {
		line
	}
	else {

		&line[s..e]
		
	}
}

// method for verifying all of the directives are valid and recognized
fn check_safe_dir(file_data: Vec<&str>) -> bool {
	let mut result = true;
	let safe_directives = [".ascii",".asciz",".align",".balign",
			".byte",".int",".double",".quad",".octa",".word",".",];
	for line in file_data.iter() {
		//println!("line: {}", line);
		//println!("check_dot_cmd: {}", check_dot_cmd(line));
		if &check_dot_cmd(&line) == line {
			result = true;
			continue;
		}
		else if !safe_directives.contains(&check_dot_cmd(&line)) {
			println!("Unrecognized safe directive.");
			result = false;
		}
	}
	return result;
}

fn parsedumptruck(mut path_string: String) -> String {

  let mut asm_file = match File::open("ext/core/asm.txt") {
    Ok(input) => input,
    Err(err) => {
			println!("Failed to parse asm.txt, Error : {}", err);
			return err.to_string();
    }
  };
	
	let mut buffer = String::new();
	match asm_file.read_to_string(&mut buffer) {
		Ok(input) => input,
		Err(err) => {
			println!("Failed to read into buffer, Error : {}", err);
			return err.to_string();
		}
	};
	let outdump: Vec::<&str> = buffer.split("\n").collect();
	if outdump.len() < 7 {
		return path_string;
	}
	let mut sliced = outdump[7..].to_vec();
	sliced.pop();
	path_string.pop();
	path_string.pop();
	let out_fname = String::from("-out.txt");
	path_string.push_str(&out_fname);
	let out_path = Path::new(&path_string);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	let mut out_handle = BufWriter::new(output_file);
	
	let mut fina = String::new();
	for i in sliced.iter() {
		let temp_vec: Vec::<&str> = i.split("\t").collect();
		fina.push_str(temp_vec[1]);
	}
	let mut donefina =fina.replace(" ", "");

	let mut i = 0;
	while i < donefina.len() {
		if i%2==0 {
			donefina.insert_str(i, "\\x");
			i+=4;
		}
	}
	
	println!("{}", donefina);
	match write!(out_handle, "{}", donefina) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to read input. Error : {}", err);
			return err.to_string();
		}	
	};
	
	return donefina;
}

fn objdumptruck() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("Couldnt objdump: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("ext/core/asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};	
	return dumptruck_str.to_string();
	
}

fn objdumptruck_ppc() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/powerpc-linux-gnu-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_ppc64() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/powerpc64-linux-gnu-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_mips() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/mips-linux-gnu-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_mips64() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/mips64-linux-gnuabi64-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_arm() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	//arguments.push("-EB");	
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/arm-none-eabi-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_arm64() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/aarch64-linux-gnu-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_riscv64() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-EB");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/riscv64-linux-gnu-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("couldnt objdump riscv64: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_sparc() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/sparc64-linux-gnu-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_avr() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	//arguments.push("-M");
	//arguments.push("intel");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("/usr/bin/avr-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn objdumptruck_riscv32() -> String {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-z");
	arguments.push("-M");
	arguments.push("no-aliases");
	arguments.push("-d");
	arguments.push("tmp.o");
	
	let output_objdump = match Command::new("riscv64-linux-gnu-objdump").args(arguments).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return e.to_string();
		}
	};
	let dumptruck_str = String::from_utf8_lossy(&output_objdump.stdout);
	
	let out_filename = String::from("asm.txt");
	let out_path = Path::new(&out_filename);
	let output_file = match File::create(out_path) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return e.to_string();
		}
	};
	
	let stderr_str = String::from_utf8_lossy(&output_objdump.stderr);
	
	if !stderr_str.is_empty() {
		print!("{}", stderr_str);
	}
	
	let mut out_handle = BufWriter::new(output_file);
	//println!("{}",dumptruck_str);
	match write!(out_handle, "{}", dumptruck_str) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to write. Error : {}", err);
			return err.to_string();
		}	
	};
	return dumptruck_str.to_string();
	
}

fn asm_x86(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	let arch_tick = String::from("-m32");
	arguments.push(&arch_tick);
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	//arguments.push("-mlittle-endian");	
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("gcc").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);

	}
}

fn asm_x64(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	let arch_tick = String::from("-m64");
	arguments.push(&arch_tick);
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("gcc").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);

	}
}

fn asm_ppc(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	arguments.push("-mlittle-endian");	
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("powerpc-linux-gnu-gcc").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_ppc64(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	arguments.push("-mlittle-endian");	
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("powerpc64-linux-gnu-gcc").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_mips(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	arguments.push("-EL");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("/usr/bin/mips-linux-gnu-gcc-10").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_mips64(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-EL");
	arguments.push("-O0");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("/usr/bin/mips64-linux-gnuabi64-gcc-11").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_arm(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	arguments.push("-mlittle-endian");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("/usr/bin/arm-none-eabi-gcc").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_arm64(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	arguments.push("-mlittle-endian");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("/usr/bin/aarch64-linux-gnu-gcc").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_riscv32(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-march=rv32i");
	arguments.push("-mabi=ilp32");
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	arguments.push("-mlittle-endian");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("riscv64-linux-gnu-gcc-11").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_riscv64(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");	
	//arguments.push("-mlittle-endian");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("/usr/bin/riscv64-linux-gnu-gcc-11").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("couldnt compile riscv64: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_sparc32(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	arguments.push("-m32");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("/usr/bin/sparc64-linux-gnu-gcc-11").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_sparc64(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	//arguments.push("-mlittle-endian");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("/usr/bin/sparc64-linux-gnu-gcc-11").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn asm_avr(path: String) {
	let mut arguments: Vec<&str> = Vec::new();
	arguments.push("-c");
	arguments.push(&path);
	arguments.push("-o");
	arguments.push("tmp.o");
	arguments.push("-O0");
	//arguments.push("-mlittle-endian");
	arguments.push("-Xlinker");
	arguments.push("--oformat-binary");
	
	let output_x86 = match Command::new("/usr/bin/avr-gcc").args(arguments).stderr(Stdio::null()).output() {
		Ok(out) => out,
		Err(e) => {
			println!("heres an error: {}", e);
			return;
		}
	};
	
	let stdout_str = String::from_utf8_lossy(&output_x86.stdout);
	let stderr_str = String::from_utf8_lossy(&output_x86.stderr);
	
	if stderr_str.is_empty() {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
		return;
	}
}

fn get_file(input: String) {
		
		// create out_file path and BufWriter.
		let out_filename = String::from("tmp.s");
		let out_path = Path::new(&out_filename);
		let output_file = match File::create(out_path) {
			Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return;
			}
		};
		let mut out_handle = BufWriter::new(output_file);
	match write!(out_handle, "{}", input) {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to read input. Error : {}", err);
			return;
		}	
	};
}

