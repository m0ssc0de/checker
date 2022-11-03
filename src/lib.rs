use std::fs;
use std::path::{Path, PathBuf};

fn all_files(dir: &Path, depth: i32) -> Option<Vec<PathBuf>>{
    if depth == 0 {
        return None
    }
    let mut entries:Vec<fs::DirEntry> = match fs::read_dir(dir) {
        Ok(v) => {
            v.filter_map(|v|match v.ok() {
                Some(v) => Some(v),
                None => None,
            }).collect::<Vec<_>>()
        },
        Err(_)=> return None,
    };
    entries.sort_by(|a, b|a.path().file_name().cmp(&b.path().file_name()));
    let mut res = Vec::new();
    for (_, e) in entries.iter().enumerate() {
        if !e.path().is_dir() {
            res.push(e.path());
        } else{
            let pb = e.path();
            let path_str = String::from(pb.as_os_str().to_str()?);
            let ps = Path::new(&path_str);
            match all_files(ps,depth-1) {
                Some(mut v)=>res.append(&mut v),
                None=>(),
            }
        }
    }
    Some(res)
}

fn range_filter(path_buf: Vec<PathBuf>, is_filename: bool) -> Vec<(i64, i64)> {
    path_buf.iter().filter_map(|pf|{
        if is_filename {
                let file_name = String::from(pf.file_name().unwrap().to_str().unwrap());
                let i = file_name.split("-").collect::<Vec<&str>>();
                if i.len() != 2 {
                    return None;
                } else {
                    return Some((i[0].parse::<i64>().unwrap(),i[1].parse::<i64>().unwrap()));
                }
        }
        match pf.parent() {
            Some(p)=> {
                let file_name = String::from(p.file_name()?.to_str().unwrap());
                let i = file_name.split("-").collect::<Vec<&str>>();
                if i.len() != 2 {
                    None
                } else {
                    Some((i[0].parse::<i64>().ok()?,i[1].parse::<i64>().unwrap()))
                }
            },
            None=>None
        }
    }).collect::<Vec<_>>()
}

fn cal_gap(ranges: Vec<(i64, i64)>) -> (Vec<(i64, i64)>, i64, Vec<(i64, i64)>, i64) {
    let mut miss_range = Vec::new();
    let mut miss_number = 0;
    let mut exist_range = Vec::new();
    let mut exist_number = 0;
    let mut previous = (-1, -1);
    for (i, current) in ranges.iter().enumerate() {
        if (current.0 == previous.0) && (current.1 == previous.1) {
            continue;
        }
        if i == 0 {
            previous = current.to_owned();
            continue;
        }
        if current.0-previous.1 != 1 {
            let miss = (previous.1 + 1, current.0 - 1);
            miss_range.push(miss);
            miss_number += miss.1 - miss.0;
            let exist = (previous.0, previous.1);
            exist_range.push(exist);
            exist_number += exist.1 - exist.0;
            previous = current.to_owned();
        } else {
            previous.1 = current.1.to_owned();
        }
    }
    if previous.0 != -1 && previous.1 != -1 {
        exist_range.push(previous);
        exist_number += previous.1 - previous.0;
    }
    (exist_range, exist_number, miss_range, miss_number)
}

pub fn gap_in_data(dir: &Path, depth: i32, range_in_filename: bool) -> (Vec<(i64, i64)>, i64, Vec<(i64, i64)>, i64){
    let a = all_files(dir, depth).unwrap();
    let b = range_filter(a, range_in_filename);
    cal_gap(b)
}