use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead};
use std::path::Path;

/*

This code can be used to massage the outputs of
the BOM CSV export and PickAndPlace CSV exports
from https://easyeda.com/ web UI into format that is
acceptable for https://jlcpcb.com/

The above statement is correct on 26 May 2020,
but I give no promises about maintaining this forever :-)

cargo run /path/to/a/csv/file.csv >massaged_destination.csv

The code autodetects whether it is a BOM or a PickAndPlace coordinate file.

*/

fn rot_adjust(rot: &str, delta: i32) -> String {
    let rot = rot.parse::<i32>().unwrap();
    let rot_adj = (rot + delta) % 360;
    eprintln!("Adjust rotation {} => {}", &rot, &rot_adj);
    return rot_adj.to_string();
}

fn fixup_rotation(footprint: &str, comment: &str, rot: &str) -> String {
    match (footprint, comment) {
        ("SENSOR-SMD_SPL06-007", "SPL06-007") => rot_adjust(rot, 180),
        ("LED-ARRAY-SMD_4P-L1.6-W1.5-BL-FD", "TJ-S1615SW6TGLCCSRGB-A5") => rot_adjust(rot, 90),
        _ => rot.to_string(),
    }
}

fn main() {
    let fname = std::env::args().skip(1).nth(0).unwrap();
    let mut file = File::open(fname).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Vec<u16> = buffer
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect();
    let buffer = buffer.as_slice();
    let str = String::from_utf16_lossy(&buffer);
    let mut lines = str.split("\n");
    let title = lines.nth(0).unwrap().replace("\u{feff}", "");
    let title_parts = title.split("\t").collect::<Vec<&str>>();
    eprintln!("TITLE: {:?}", &title_parts);

    let mut output = String::new();

    let is_bom = title_parts[3] == "Footprint";
    let is_pap = title_parts[6] == "Pad X";
    assert!(is_bom || is_pap);

    if is_pap {
        output.push_str(&format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
            title_parts[0], title_parts[2], title_parts[3], title_parts[8], title_parts[9]
        ));
    }
    if is_bom {
        output.push_str(&format!(
            "\"{}\",\"{}\",\"{}\",\"{}\"\n",
            "Comment", title_parts[2], title_parts[3], "LCSC Part #（optional）"
        ));
    }

    for line in lines {
        let line_parts = line
            .split("\t")
            .map(|x| x.replace("\"", ""))
            .collect::<Vec<String>>();
        if is_pap {
            if line_parts.len() < 9 {
                continue;
            }
            let layer = if line_parts[8] == "B" {
                "Bottom"
            } else {
                "Top"
            };
            let rotation = fixup_rotation(&line_parts[1], &line_parts[10], &line_parts[9]);
            output.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                line_parts[0], line_parts[2], line_parts[3], layer, &rotation
            ));
        }
        if is_bom {
            if line_parts.len() < 10 {
                continue;
            }
            output.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\"\n",
                line_parts[1], line_parts[2], line_parts[3], line_parts[8]
            ));
        }
        eprintln!("X: {:?}", &line_parts);
    }
    println!("{}", output);
}
