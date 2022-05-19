fn found_tss_identifier(buf: &Vec<u8>, file_pos: usize) -> bool {
    if buf[file_pos] == 0 && buf[file_pos + 1] == 1 &&
        buf[file_pos + 2] == 0 && buf[file_pos + 3] == 0 &&
        buf[file_pos + 4] == 15 && buf[file_pos + 5] == 133 &&
        buf[file_pos + 6] == 137 && buf[file_pos + 7] == 0  {
            true
        }
    else {
        false
    }
}

fn found_other_identifier(buf: &Vec<u8>, file_pos: usize) -> bool {
    if buf[file_pos] == 116 && buf[file_pos + 1] == 11 && 
        buf[file_pos + 2] == 185 && buf[file_pos + 3] == 1  {
            true
        }
    else {
        false
    }
}

fn check_tss(buf: &Vec<u8>) -> Result<usize, ()> {
    let mut file_pos = 0;
    let mut found = false;
    while !found_tss_identifier(&buf, file_pos) && file_pos < buf.len()-7 {
        file_pos += 1;
    }
    if file_pos < buf.len() && found_tss_identifier(&buf, file_pos) {
        found = true;
    }
    
    match found {
        true => Ok(file_pos),
        _ => Err(())
    }
}

fn check_other(buf: &Vec<u8>) -> Result<usize, ()> {
    let mut file_pos = 0;
    let mut found = false;
    while !found_other_identifier(&buf, file_pos) && file_pos < buf.len()-7 {
        file_pos += 1;
    }
    if file_pos < buf.len() && found_other_identifier(&buf, file_pos) {
        found = true;
    }
    
    match found {
        true => Ok(file_pos),
        _ => Err(())
    }
}

pub enum GameType {
    TSS,
    Other
}

pub fn get_game_type(buf: &Vec<u8>) -> Option<(GameType, usize)> {
    match check_tss(&buf) {
        Ok(file_pos) => Some((GameType::TSS, file_pos)),
        _ => {
            match check_other(&buf) {
                Ok(file_pos) => Some((GameType::Other, file_pos)),
                _ => None
            }
        }
    }
}

pub fn patch_tss_buf(buf: &mut Vec<u8>, file_pos: usize) {
    buf[file_pos + 4] = 144;
    buf[file_pos + 5] = 144;
    buf[file_pos + 6] = 144;
    buf[file_pos + 7] = 144;
    buf[file_pos + 8] = 144;
    buf[file_pos + 9] = 144;
}

pub fn patch_other_buf(buf: &mut Vec<u8>, file_pos: usize) {
    buf[file_pos] = 235;
}