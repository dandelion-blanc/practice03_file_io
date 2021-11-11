/* practice03 by rust(cargo)
 * 		written by Matsumoto Kazuki
 *
 *
 *
 */

use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write, BufReader, BufWriter, stdout};



fn string_line(s: &String) -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() -> Result<(), Box<std::error::Error>>
{
	let args: Vec<String> = env::args().collect();										//コマンドライン引数に相当
	println!("practice03 start!");

// file input
	println!("file loading...");

	println!(" by iterator");																	//イテレーターによる行入力(文字列)
	let infile = BufReader::new(File::open(".\\input.txt").expect("Unable to open"));
	for line in infile.lines()
//	for line in BufReader::new(File::open(".\\input.txt").expect("Unable to open")).lines()				こちらでも可
	{
		let str = line.expect("reader is failed.");
		println!("  { }", str);
    }
	println!(" ");

	println!(" by read_line");																//通常行入力(文字列)
	let mut infile = BufReader::new(File::open(".\\input.txt").expect("Unable to open"));
    let mut buf = String::new();

    while infile.read_line(&mut buf).expect("reader is failed.") > 0
    {
        println!("  { }", buf.trim());
        buf.clear();																			//バッファ初期化
    }
    println!(" ");

	println!(" by all file");																	//全データ取り込み
	let infile = fs::read_to_string(".\\input.txt").expect("Unable to open");
	let mut lines : usize = 0;
	let mut index_s = 0;
	for (i, &item) in infile.as_bytes().iter().enumerate()								//行単位に分割
	{
		if item == b'\n'
		{
			let buf = &infile[index_s..i];
			lines += 1;
			index_s = i + 1;

			println!("  { }. { }", lines, buf);
        }
    }

// console input
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");

// file output
	println!("file uploading...");

	let buf = "test text".to_string();
	let mut outfile = File::create(".\\output.txt").expect("Unable to open");

	writeln!(outfile, "{ }", args[0]).expect("writeln! is failed.");
	writeln!(outfile, "practice03 output").expect("writeln! is failed.");
//    let buf = BufReader::new(io::stdin()).lines().collect::<io::Result<Vec<String>>>()?.join("\n");		//常時入力受付

	writeln!(outfile, " by write_all").expect("writeln! is failed.");
	outfile.write_all(buf.as_bytes()).expect("write_all is failed.");											//バイト列の出力

	writeln!(outfile, " by write!").expect("writeln! is failed.");
	write!(outfile, "  {}", buf).expect("write! is failed.");														// write!マクロ：改行なし

	writeln!(outfile, " by writeln!").expect("writeln! is failed.");
	writeln!(outfile, "  {}", buf).expect("writeln! is failed.");													// writeln!マクロ：改行内包

// BufWriter
	let mut buf = BufWriter::new(File::create(".\\output_.txt").expect("Unable to open"));
					//BufWriter::new(outfile);																		これも可
	buf.write_all(input.as_bytes()).expect("Bufwriter write_all is failed.");

// buffering
																														//file出力もlockできるとさらに高速化の可能性
	writeln!(outfile, " buffering test\n").expect("writeln! is failed.");

	let out = stdout();
	let mut out = BufWriter::new(out.lock());
	let yes =
				{
					let mut s = String::with_capacity(4096);
					for i in 0..2048
					{
						s += &(i.to_string() + "\n");
					}
					s
				};
    let rest =
				{
					let mut s = String::with_capacity(4096);
					for _ in 0..(10_000_000 % 2048)
					{
						s += "rest\n";
					}
					s
				};

    for _ in 0..(10_000_000 / 2048)
    {
        out.write_all(yes.as_bytes()).unwrap();
        outfile.write_all(yes.as_bytes()).unwrap();
    }
    out.write_all(rest.as_bytes()).unwrap();
	outfile.write_all(rest.as_bytes()).unwrap();

    outfile.flush().expect("flush is failed.");
    Ok(())
}
