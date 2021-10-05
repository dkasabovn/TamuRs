use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::io::BufRead;

pub struct TamuEntry {
    pub converted_name: String,
    pub property_value: String,
}

pub fn find_tamu() -> std::io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    for entry in fs::read_dir(current_dir)? {
        let file_entry = entry?;
        let file_type = file_entry.file_type()?;
        if file_type.is_file() {
            let file_name = file_entry.file_name();
            let file_path = Path::new(file_name.as_os_str());
            match file_path.extension() {
                Some(v) => {
                    if v == "tamu" {
                        return Ok(file_path.to_path_buf())
                    }
                },
                None => ()
            }

        }
    }
    return Err(io::Error::new(io::ErrorKind::NotFound, "Could not find a .tamu file in the current directory"))
}

pub fn parse_tamu(buf: PathBuf) -> std::io::Result<Vec<TamuEntry>> {
    let file = File::open(buf.as_path())?;
    let reader = BufReader::new(file);
    let mut output_vec: Vec<TamuEntry> = Vec::new();

    for line in reader.lines() {
        let read_line = line?;
        let mut read_line_iter = read_line.split_whitespace();
        let c_name: String;
        {
            let color_name = read_line_iter.next();
            match color_name {
                Some(v) => {
                    c_name = v.to_string();
                },
                None => {
                    eprintln!("You should not skip lines within a .tamu file for my sake");
                    continue;
                }
            }
        }

        let c_value: String;
        {
            let color_value = read_line_iter.next();
            match color_value {
                Some(v) => {
                    c_value = v.to_string();
                },
                None => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Pal file is misformatted. Each line should have: Color [space] Value(Hex, HSL, RGB)"));                    
                }
            }
        }

        let tamu_entry = TamuEntry {
            converted_name: c_name,
            property_value: c_value
        };
        output_vec.push(tamu_entry)
    }

    Ok(output_vec)
}

pub fn to_css_variables_file(tamu_entries: Vec<TamuEntry>) -> std::io::Result<()> {
    let generated_file_path = env::current_dir()?.join("generated.css");
    let generated_file = match File::create(generated_file_path.as_path()) {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };
    let mut  writer = BufWriter::new(generated_file);
    
    writer.write(":root {\n".as_bytes())?;
    for tamu_entry in tamu_entries {
        writer.write(format!("\t--{}: {};\n", tamu_entry.converted_name, tamu_entry.property_value).as_bytes())?;
    }

    writer.write("}".as_bytes())?;

    Ok(())
}

pub fn to_tailwind_config(tamu_entries: Vec<TamuEntry>) -> std::io::Result<()> {
    let generated_file_path = env::current_dir()?.join("generated.js");
    let generated_file = match File::create(generated_file_path.as_path()) {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };
    let mut  writer = BufWriter::new(generated_file);
    
    writer.write("module.exports = {\n\ttheme: {\n\t\tcolors: {\n".as_bytes())?;
    for tamu_entry in tamu_entries {
        writer.write(format!("\t\t\t\'{}\': \'{}\',\n", tamu_entry.converted_name, tamu_entry.property_value).as_bytes())?;
    }

    writer.write("\t\t}\n\t}\n}".as_bytes())?;

    Ok(())
}

pub fn write_to_css() {
    let path_to_tamu = match find_tamu() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Err: {}", e);
            return;
        }
    };
    let entries = match parse_tamu(path_to_tamu) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Err: {}", e);
            return;
        }
    };
    match to_css_variables_file(entries) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Err: {}", e);
            return;
        }
    };
}

pub fn write_to_tailwind() {
    let path_to_tamu = match find_tamu() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Err: {}", e);
            return;
        }
    };
    let entries = match parse_tamu(path_to_tamu) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Err: {}", e);
            return;
        }
    };
    match to_tailwind_config(entries) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Err: {}", e);
            return;
        }
    };
}