// AssemblerDisassembler
// QVLx Labs
// Authors: Matzr3lla, m0nZSt3r, $t@$h
/*
	 This disassembler will take hex file data cooresponding to one of the 
	 following instruction set architectures (x86,x64,ARM(32), ARM(64), MIPS(32), MIPS(64),
	 PowerPC(32), PowerPC(64), AVR(32), RISCV(32), RISCV(64), Sparc(32),Sparc(64)) and dump the instruction output to the terminal or the
	 file "dis_inst.txt".
 */
use binascii::hex2bin;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::str;
use std::env;
use std::io::Read;

fn main() {
	let args:Vec<String> = env::args().collect();
	if args.len() == 1 || args[1] == "-h" {
		println!("Usage: das <isa> <hex file>");
		return;
	}

	let isa = &args[1].trim();
	let data = &args[2].trim();
	let assem_path = file_flag(data);
	let m_isa : &str = &(isa.trim()).to_lowercase().to_string();
	let mut dump:Vec<&str> = match m_isa {
		"arm32" =>	vec!["/usr/bin/arm-none-eabi-objdump","-z","--no-show-raw-insn","-EB","-D"],
		"arm64" =>  vec!["/usr/bin/aarch64-linux-gnu-objdump","-z","--no-show-raw-insn","-EL","-D"],
		"avr" => vec!["/usr/bin/avr-objdump","-z", "--no-show-raw-insn","-D"],
		"riscv32" => vec!["/usr/bin/riscv64-linux-gnu-objdump","-z","-M","no-aliases", "--no-show-raw-insn","-D"],
		"riscv64" => vec!["/usr/bin/riscv64-linux-gnu-objdump","-z", "-EL","--no-show-raw-insn","-D"],
		"ppc32" =>  vec!["/usr/bin/powerpc-linux-gnu-objdump","-z","powerpc:common32","--no-show-raw-insn","-D"],
		"ppc64" => 	vec!["/usr/bin/powerpc64-linux-gnu-objdump","-z","powerpc:common64","--no-show-raw-insn","-D"],
		"mips32" => vec!["/usr/bin/mips-linux-gnu-objdump","-z","-mmips:isa32","--no-show-raw-insn","-EB","-D"],
		"mips64" => vec!["/usr/bin/mips64-linux-gnuabi64-objdump","-z","-mmips:isa64","-EB","-mmips:isa64","--no-show-raw-insn","-D"],
		"sparc32" => vec!["/usr/bin/sparc64-linux-gnu-objdump","-z","-EL","--no-show-raw-insn","-D"],
		"sparc64" => vec!["/usr/bin/sparc64-linux-gnu-objdump","-z","-EL","--no-show-raw-insn","-D"],
		"x86" => vec!["objdump","-z","-b","binary","-m","i386","--no-show-raw-insn","-D"],
		"x64" => vec!["objdump","-z","-b","binary","-m","i386:x86-64","--no-show-raw-insn","-D"],
		unknown => {
			println!("Error, {} is an invalid instruction set architecture",unknown);
			return;
		}
	};
	let isa_dump = dump.remove(0);
	execute(&convert_bin(&assem_path, &m_isa), &dump, &isa_dump);

}
/*
	 Hex data read from file input and converted to binary format.
 */
fn file_flag(path: &str) -> &str {
	let mut file = match File::open(path.trim()) {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to open specified file. Error: {}",err);
				return "file_error"
			}
	};
	let mut file_data = String::new();
	match file.read_to_string(&mut file_data) {
		Ok(byte_num) => byte_num,
			Err(err) => {
				println!("Unable to read file data to string. Error: {}",err);
				return "file_error"

			}
	};

	let mut output_buf = vec![0u8;file_data.len()*2];

	let bin = match hex2bin(((file_data.trim()).replace(" ","")).replace("\n","").as_bytes(), &mut output_buf) {
		Ok(bin) => bin,
			Err(err) => {
				println!("Unable to convert string hex to binary from file. Error: {:?}",err);
				return "file_error"
			}
	};



	let mut inst_file = match File::create("inst.o") {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to create \"inst.o\" file. Error: {}",err);
				return "file_error"
			}
	};

	match inst_file.write(bin) {
		Ok(num_bytes) => num_bytes,
			Err(err) => {
				println!("Unable to write binary to file \"inst.o\". Error: {}",err);
				return "file_error"
			}
	};
	"inst.o"
}
/*
	 Instruction portion of the object dump is extracted and written to file
	 or printed to terminal based on users input.
 */
fn format_output(output: Vec<u8>)  {
	let output_str = match str::from_utf8(&output) {
		Ok(string) => string,
			Err(err) => {
				println!("Unable to convert output from utf8 to &str. Error: {}",err);
				return;
			}
	};

	let mut output_vec : Vec<&str> = output_str.split('\n').collect();
	output_vec = output_vec[7..].to_vec();
	output_vec.pop();
	for line in output_vec.iter() {
		let _temp_vec: Vec<&str> = line.split('\t').collect();
		let pos = match line.chars().position(|c| c == ':') {
			Some(count) => count + 1,
				None => {
					println!("Unable to get position for colon in instruction.");
					return;
				}
		};
		let instruc : &str= &line[pos..];
		println!("{}",instruc);
	}

}

/*
	 Converts binary file to appropriate file format depending on the 
	 instruction set architecture.
 */
fn convert_bin(path: &str, isa: &str) -> String {
	if isa == "x86" || isa == "x64" {
		return "inst.o".to_string()
	}
	match isa {
		"ppc32" => {
			match Command::new("powerpc-linux-gnu-objcopy")
				.args(["-I","binary","-O","elf32-powerpc","--reverse-bytes=4",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		"ppc64" =>  {
			match Command::new("powerpc64-linux-gnu-objcopy")
				.args(["-I","binary","-O","elf64-powerpc","--reverse-bytes=4",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		"riscv32" => {
			match Command::new("riscv64-linux-gnu-objcopy")
				.args(["-I","binary","-O","elf32-littleriscv","--reverse-bytes=4",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		"riscv64" => {
			match Command::new("riscv64-linux-gnu-objcopy")
				.args(["-I","binary","-O","elf64-littleriscv","--reverse-bytes=4",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		"avr" => {
			match Command::new("avr-objcopy")
				.args(["-I","binary","-O","elf32-avr",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		"sparc32" => {
			match Command::new("sparc64-linux-gnu-objcopy")
				.args(["-I","binary","-O","elf32-sparc","--reverse-bytes=4",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		"sparc64" => {
			match Command::new("sparc64-linux-gnu-objcopy")
				.args(["-I","binary","-O","elf64-sparc","--reverse-bytes=4",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}

		"mips32" => {
			match Command::new("mips-linux-gnu-objcopy")
				.args(["-I","binary","-O","elf32-tradlittlemips",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}

		"mips64" => {
			match Command::new("mips64-linux-gnuabi64-objcopy")
				.args(["-I","binary","-O","elf64-tradlittlemips",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		"arm32" => {
			match Command::new("arm-none-eabi-objcopy")
				.args(["-I","binary","-O","elf32-littlearm-fdpic","--reverse-bytes=4",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		"arm64" => {
			match Command::new("aarch64-linux-gnu-objcopy")
				.args(["-I","binary","-O","elf64-littleaarch64","--reverse-bytes=4",path,"inst.bin"])
				.status() {
					Ok(output) => output,
						Err(err) => {
							println!("Unable to convert binary file to elf32-powerpc. Error: {}",err);
							return "isa error".to_string()
						}
				};
		}
		_ => return "error".to_string()
	}
	"inst.bin".to_string()
}


/*
	 This method executed the the objdump associated with the desired instruction
	 set architecture and passes the output to the format_output function.
 */
fn execute(path: &str,dump: &Vec<&str>, isa: &str) {
	let output;
	output = match Command::new(isa)
		.args(dump)
		.arg(path.trim())
		.output() {
			Ok(output) => output,
				Err(err) => {
					println!("Unable to execute objdump on specified file.
							Error: {}",err);
					return;
				}
		};
	format_output(output.stdout);
}
