use std::fs::File;
use std::io::Read;


pub fn open_file(path: String) -> File
{
        let _f = File::open(path).expect("Something went wrong opening the file!");
        return _f;
    }

pub fn store_file(mut f: File) -> Vec<u8>
{
        let mut _buffer = Vec::new();
        f.read_to_end(&mut _buffer).expect("Something went wrong reading the file!");
        return _buffer;
    }

pub fn print_rom(mut _rom: Vec<u8>)
{
        for (i, element) in _rom.iter().enumerate()
        {
            println!("{:#04X}: {:#04X}", i, element);
        }
    }
