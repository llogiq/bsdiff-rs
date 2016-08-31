use std::io::Read;
use bzip2::read::BzDecoder;
use errors::*;
use brotli;

fn to_i64(buf: [u8; 8]) -> i64 {
    ((buf[7] as u64) << 54 | (buf[6] as u64) << 48 | (buf[5] as u64) << 40 |
     (buf[4] as u64) << 32 | (buf[3] as u64) << 24 | (buf[2] as u64) << 16 |
     (buf[1] as u64) << 8  | (buf[0] as u64)) as i64
}

// add `offset` to `pos`, check for under- or overflow
fn checked(pos: usize, offset: i64, len: usize) -> Result<usize> {
    println!("current: {} offset: {} len: {}", pos, offset, len);
    if offset < 0 {
        let o = (-offset) as usize;
        if o > pos { Err("underflow".into()) } else { Ok(pos - o) }
    } else {
        let o = offset as usize;
        if pos + o > len { Err("overflow".into()) } else { Ok(pos + o) }
    }
}

fn do_patch<R: Read + Sized>(old: &[u8], cur: &mut [u8], patch: &mut R) -> Result<()> {
    let (oldsize, cursize) = (old.len(), cur.len());
    let (mut oldpos, mut curpos) = (0, 0);
    let mut buf = [0u8; 8];
    let mut ctrl = [0i64; 3];

    while curpos < cursize {
        for mut c in &mut ctrl {
            try!(patch.read_exact(&mut buf));
            *c = to_i64(buf);
        }
        print!("cur/0 ");
        let curend = try!(checked(curpos, ctrl[0], cursize));
        try!(patch.read_exact(&mut cur[curpos..curend]));
        for (o, mut c) in old[oldpos..].iter().zip(cur[curpos..curend].iter_mut()) {
            *c += *o;
        }
        curpos = curend;
        print!("old/0 ");
        oldpos = try!(checked(oldpos, ctrl[0], oldsize));
        print!("cur/1 ");
        let curend = try!(checked(curpos, ctrl[1], cursize));
        try!(patch.read_exact(&mut cur[curpos..curend]));
        curpos = curend;
        print!("old/2 ");
        oldpos = try!(checked(oldpos, ctrl[2], oldsize));
    }
    if curpos == cursize { Ok(()) } else { Err("missed some bytes".into()) }
}

pub enum HeaderType {
    Endsley,
    Llogiq,
}

fn match_header(data: &[u8]) -> Result<HeaderType> {
    if data == b"ENDSLEY/BSDIFF43" {
        Ok(HeaderType::Endsley)
    } else if data == b"LLOGIQ/BSDIFF01" {
        Ok(HeaderType::Llogiq)
    } else {
        Err("Unsupported Header".into())
    }
}

pub fn patch(old: &[u8], patch: &mut Read) -> Result<Vec<u8>> {
    // just use mut ref to reduce monomorphization bloat
    let mut header = [0u8; 16];
    try!(patch.read_exact(&mut header));
    let headertype = try!(match_header(&header));
    let mut buf = [0u8; 8];
    try!(patch.read_exact(&mut buf));
    let curlen = to_i64(buf) as usize;
    println!("curlen = {}", curlen);
    let mut cur = vec![0; curlen];
    match headertype {
        HeaderType::Endsley => {
            let mut bz = BzDecoder::new(patch);
            try!(do_patch(old, &mut cur, &mut bz));
        }
        HeaderType::Llogiq => {
            let mut br = brotli::Decompressor::new(patch, 16384);
            try!(do_patch(old, &mut cur, &mut br));
        }
    }
    Ok(cur)
}
