// #![allow(unused_imports)]
// #![allow(unused_variables)]

use heck::CamelCase;
use std::path::PathBuf;
use std::{env, fs, io::BufWriter};
use std::{error::Error, io::Write};
use std::{fs::OpenOptions, io::BufRead};

use aa_name::AminoAcid;
use std::convert::TryFrom;
use std::io::{self, BufReader};
use std::str::FromStr;

use nom::{Finish, IResult};

use nom::one_of;

#[macro_use]
extern crate nom;

fn generate_starting_comment(
    mut writer: impl std::io::Write,
) -> std::result::Result<(), std::io::Error> {
    writeln!(writer, "// generated by /build.rs")?;
    Ok(())
}

fn generate_struct_matrices(
    paths: Vec<PathBuf>,
    mut writer: impl std::io::Write,
) -> std::result::Result<(), std::io::Error> {
    for path in paths {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        writeln!(
            writer,
            r#"
/// implementation generated from <ftp://ftp.ncbi.nih.gov/blast/matrices/{stem}>
///
/// ## Examples
/// ```rust
/// use aa_similarity::{{{name}, Similarity, AminoAcid}};
///
/// let d: i16 = {name}::similarity(AminoAcid::Glycine, AminoAcid::Arginine);
///
/// assert_eq!(
///    {name}::similarity(AminoAcid::Glycine, AminoAcid::AsparticAcid),
///    {name}::similarity(AminoAcid::AsparticAcid, AminoAcid::Glycine)
/// );
/// ```
pub struct {name};
            "#,
            stem = stem,
            name = stem.to_camel_case()
        )?;
    }
    Ok(())
}

fn generate_enum_matrices(
    paths: Vec<PathBuf>,
    mut writer: impl std::io::Write,
) -> std::result::Result<(), std::io::Error> {
    writeln!(
        writer,
        r#"
/// One variant per matrix    
pub enum SubstitutionMatrix {{"#
    )?;
    for path in paths.clone() {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        writeln!(
            writer,
            r#"    {name}({name}),"#,
            name = stem.to_camel_case()
        )?;
    }
    writeln!(writer, "}}")?;

    Ok(())
}

fn generate_impl_matrices(
    path: &PathBuf,
    records: Vec<MatrixRecord>,
    mut writer: impl std::io::Write,
) -> std::result::Result<(), std::io::Error> {
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let name = stem.to_camel_case();

    let mut string_writer = Vec::new();
    writeln!(
        string_writer,
        r#"
impl Similarity for {name} {{
    fn similarity(lhs: AminoAcid, rhs: AminoAcid) -> i16 {{
        match (lhs, rhs) {{"#,
        name = name
    )?;

    for record in records {
        let aa = record.aa;

        for value in record.values.iter() {
            writeln!(
                string_writer,
                r#"           (AminoAcid::{lhs:?}, AminoAcid::{rhs:?}) => {value}_i16,"#,
                lhs = aa,
                rhs = value.0,
                value = value.1
            )?;
        }
    }

    writeln!(
        string_writer,
        r#"        }}
    }}
}}
// end of {name}
"#,
        name = name
    )?;
    write!(writer, "{}", std::str::from_utf8(&string_writer).unwrap())?;
    writer.flush()?;
    Ok(())
}

#[derive(Debug)]
struct MatrixRecord {
    aa: AminoAcid,
    values: [(AminoAcid, i16); 20],
}

named!(space<&str, &str>, take_while1!(|c| c == ' '));
named!(amino_acid<&str, char>, one_of!("ARNDCQEGHILKMFPSTWYV"));
named!(value<&str, i16>, map_res!(take_until!(" "), |x| i16::from_str_radix(x, 10)));

fn parse_matrix_record(input: &str) -> IResult<&str, MatrixRecord> {
    let (input, aa_line) = amino_acid(input).unwrap();

    let aa: AminoAcid = AminoAcid::try_from(aa_line).unwrap();

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let alanine = (AminoAcid::Alanine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let arginine = (AminoAcid::Arginine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let asparagine = (AminoAcid::Asparagine, score);
    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let aspartic_acid = (AminoAcid::AsparticAcid, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let cysteine = (AminoAcid::Cysteine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let glutamine = (AminoAcid::Glutamine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let glutamic_acid = (AminoAcid::GlutamicAcid, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let glycine = (AminoAcid::Glycine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let histidine = (AminoAcid::Histidine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let isoleucine = (AminoAcid::Isoleucine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let leucine = (AminoAcid::Leucine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let lysine = (AminoAcid::Lysine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let methionine = (AminoAcid::Methionine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let phenylalanine = (AminoAcid::Phenylalanine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let proline = (AminoAcid::Proline, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let serine = (AminoAcid::Serine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let threonine = (AminoAcid::Threonine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let tryptophan = (AminoAcid::Tryptophan, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let tyrosine = (AminoAcid::Tyrosine, score);

    let (input, _) = space(input).unwrap();
    let (input, score) = value(input).unwrap();
    let valine = (AminoAcid::Valine, score);

    let values = [
        alanine,
        arginine,
        asparagine,
        aspartic_acid,
        cysteine,
        glutamine,
        glutamic_acid,
        glycine,
        histidine,
        isoleucine,
        leucine,
        lysine,
        methionine,
        phenylalanine,
        proline,
        serine,
        threonine,
        tryptophan,
        tyrosine,
        valine,
    ];

    Ok((&input, MatrixRecord { aa, values }))
}

impl FromStr for MatrixRecord {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_matrix_record(s).finish() {
            Ok((_remaining, name)) => Ok(name),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=/assets/");
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let matrix_dir = root.join("data");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let dest_path = out_dir.join("generated_matrices.rs");
    let dest_file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(dest_path)
        .unwrap();

    dest_file.set_len(0)?;

    let mut dest_writer = BufWriter::new(dest_file);

    let matrix_files = fs::read_dir(matrix_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    generate_starting_comment(&mut dest_writer)?;

    generate_struct_matrices(matrix_files.clone(), &mut dest_writer)?;
    generate_enum_matrices(matrix_files.clone(), &mut dest_writer)?;

    dest_writer.flush()?;

    for path in matrix_files.iter() {
        let mut matrix_records2 = Vec::new();

        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)
            .unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                match &line[..1] {
                    "#" | " " | "*" | "X" | "Z" | "B" => {}
                    _ => {
                        let record = MatrixRecord::from_str(&line)?;
                        // dbg!(&record);
                        matrix_records2.push(record);
                    }
                }
            }
        }

        generate_impl_matrices(path, matrix_records2, &mut dest_writer)?;
        dest_writer.flush()?;
    }

    Ok(())
}
