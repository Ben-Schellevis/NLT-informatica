use std::io;

use colored::*;
use std::collections::HashMap;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

fn dubbeltrans(arg: String) -> String {
    let start_zoeken_start = vec!["AUAU".to_owned()];
    let start = vec!["AUG".to_owned()];
    let stop = vec!["UAA".to_owned(), "UAG".to_owned(), "UGA".to_owned()];
    let mut res = "".to_string();
    let mut _final = "".to_string();
    let old_chars: Vec<char> = arg.clone().chars().collect();
    let chars: Vec<char> = rna(arg).chars().collect();
    let mut rna: String = "".to_owned();
    let mut i = 0;
    let mut start_zoeken_start_found = false;
    let mut start_found = false;
    while i < chars.len() {
        if !start_zoeken_start_found {
            let x: String = chars[i..i + 4]
                .to_vec()
                .iter()
                .cloned()
                .map(String::from)
                .collect();
            let old_x: String = old_chars[i..i + 4]
                .to_vec()
                .iter()
                .cloned()
                .map(String::from)
                .collect();
            if start_zoeken_start.contains(&x) {
                start_zoeken_start_found = true;
                rna = format!("{}{}", rna, old_x.blue());
                i += 4;
            } else {
                rna.push(old_chars[i]);
                i += 1;
            }
        } else if !start_found {
            let x: String = chars[i..i + 3]
                .to_vec()
                .iter()
                .cloned()
                .map(String::from)
                .collect();
            let old_x: String = old_chars[i..i + 3]
                .to_vec()
                .iter()
                .cloned()
                .map(String::from)
                .collect();
            if x == start[0] {
                rna = format!("{}{}", rna, old_x.green());
                let eiwit = eiwit(x);
                res.push_str(&eiwit);
                start_found = true;
                i += 3;
            } else {
                rna.push(old_chars[i]);
                i += 1;
            }
        } else {
            let x: String = chars[i..i + 3]
                .to_vec()
                .iter()
                .cloned()
                .map(String::from)
                .collect();
            let old_x: String = old_chars[i..i + 3]
                .to_vec()
                .iter()
                .cloned()
                .map(String::from)
                .collect();
            if stop.contains(&x) {
                let mut to_print = "".to_owned();
                rna = format!("{}{}", rna, old_x.red());
                to_print = format!("{}{}", to_print, rna);
                to_print.push_str(":");
                to_print.push_str("\n");
                to_print.push_str(&res.clone());
                to_print.push_str("\n\n");
                _final = format!("{}{}", _final,to_print);
                start_zoeken_start_found = false;
                start_found = false;
                rna = "".to_owned();
                res = "".to_owned();
            } else {
                rna = format!("{}{}", rna, old_x);
                let eiwit = eiwit(x);
                res.push_str("-");
                res.push_str(&eiwit);
            }
            i += 3;
        }
    }
    _final
}

fn eiwit(arg: String) -> String {
    let f = "Fenylalanine";
    let l = "Leucine";
    let i = "Isoeucine";
    let v = "Valine";
    let s = "Serine";
    let p = "Proline";
    let t = "Treonine";
    let a = "Alanine";
    let y = "Tyrosine";
    let h = "Histidine";
    let q = "Glutamine";
    let n = "Asparagine";
    let k = "Lysine";
    let d = "Asparaginezuur";
    let e = "Glutaminezuur";
    let c = "Cysteine";
    let w = "Tryptofaan";
    let r = "Arginine";
    let g = "Glycine";
    let m = "Methionine";

    let eiwitten: HashMap<_, _> = collection! {
        "UUU" => f,
        "UUC" => f,
        "UUA" => l,
        "AUG" => m,
        "UUG" => l,
        "CUU" => l,
        "CUC" => l,
        "CUA" => l,
        "CUG" => l,
        "AUU" => i,
        "AUC" => i,
        "AUA" => i,
        "GUU" => v,
        "GUC" => v,
        "GUA" => v,
        "GUG" => v,
        "UCU" => s,
        "UCC" => s,
        "UCA" => s,
        "UCG" => s,
        "CCU" => p,
        "CCC" => p,
        "CCA" => p,
        "CCG" => p,
        "ACU" => t,
        "ACC" => t,
        "ACA" => t,
        "ACG" => t,
        "GCU" => a,
        "GCC" => a,
        "GCA" => a,
        "GCG" => a,
        "UAU" => y,
        "UAC" => y,
        "CAU" => h,
        "CAC" => h,
        "CAA" => q,
        "CAG" => q,
        "AAU" => n,
        "AAC" => n,
        "AAA" => k,
        "AAG" => k,
        "GAU" => d,
        "GAC" => d,
        "GAA" => e,
        "GAG" => e,
        "UGU" => c,
        "UGC" => c,
        "UGG" => w,
        "CGU" => r,
        "CGC" => r,
        "CGA" => r,
        "CGG" => r,
        "AGU" => s,
        "AGC" => s,
        "AGA" => r,
        "AGG" => r,
        "GGU" => g,
        "GGC" => g,
        "GGA" => g,
        "GGG" => g,
    };

    eiwitten.get(arg.as_str().trim()).unwrap().to_string()
}

fn rna(arg: String) -> String {
    let mut rna: Vec<char> = vec![];
    for ch in arg.trim().chars() {
        match ch.to_ascii_lowercase() {
            't' => rna.push('A'),
            'a' => rna.push('U'),
            'c' => rna.push('G'),
            'g' => rna.push('C'),
            _ => todo!(),
        }
    }
    rna.iter().collect()
}

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let args: Vec<String> = buffer.split(" ").map(String::from).collect();
        let funcs: HashMap<_, fn(String) -> String> = collection! {
            "eiwit" => eiwit as fn(String) -> String,
            "rna" => rna as fn(String) -> String,
            "trans" => dubbeltrans as fn(String) -> String,
        };

        if let Some(func) = funcs.get(args[0].to_ascii_lowercase().as_str()) {
            let code = func(args[1].clone());
            println!("{}", code);
        }
    }
}
