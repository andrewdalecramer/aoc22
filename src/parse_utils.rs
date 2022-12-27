

pub type ByteIterator<'a> = std::iter::Peekable<std::iter::Enumerate<std::str::Bytes<'a>>>;

#[allow(dead_code)]
pub fn get_byte_iterator(source: &str) -> ByteIterator {
    source.bytes().enumerate().peekable()
}

#[allow(dead_code)]
pub fn is_digit(byte: u8) -> bool {
    b'0' <= byte && byte <= b'9'
}

#[allow(dead_code)]
pub fn consume_white_space(it: &mut ByteIterator) {
    loop {
        match it.peek() {
            Some((_, b'\n')) => { }
            Some((_, b'\t')) => { }
            Some((_, b' ')) => { }
            _ => { return; }
        }
        it.next();
    }
}

#[allow(dead_code)]
pub fn parse_number(src: &str, it: &mut ByteIterator) -> isize {
    let start = match it.peek() {
        Some((index, val)) => {
            if is_digit(*val) {
                *index
            } else if *val == b'-' {
                // Handle negation here
                it.next();
                return -parse_number(src, it);
            } else {
                panic!("parse_number couldn't find the number");
            }
        },
        None => {
            panic!("parse_number couldn't find the number");
        },
    };
    it.next();

    let end = loop {
        match it.peek() {
            Some((index, val)) => {
                if !is_digit(*val) {
                    break *index;
                }
                it.next();
            },
            None => {
                break usize::MAX;
            }
        }
    };
    
    let substr = 
        if end == usize::MAX {
            &src[start..]
        } else {
            &src[start..end]
        };

    substr.parse().expect("Failed to parse number")
}


/// Consumes sequence from iterator it
///
/// If sequence is consumed, return true
/// If first byte doesn't match, return false
/// If any other byte doesn't match, panic
#[allow(dead_code)]
pub fn consume_sequence(it: &mut ByteIterator, sequence: &str) -> bool {
    let mut seq_it = sequence.bytes();
    let seq_byte = seq_it.next().expect("sequence must be at least one character long");
    match it.peek() {
        Some((_, byte)) => {
            if *byte != seq_byte {
                return false;
            }
            it.next();
        },
        None => {
            return false;
        }
    };

    let mut loc = 0;
    loop {
        let seq_byte = match seq_it.next() {
            Some(v) => v,
            None => { return true; },
        };
        let it_byte = match it.next() {
            Some((_, byte)) => byte,
            None => {
                panic!("Unexpected end of input consuming sequence \"{}\"", sequence);
            }
        };
        if seq_byte != it_byte {
            panic!(
                "Unexpected byte \"{}\" consuming sequence \"{}\" at position {}",
                std::str::from_utf8(&[it_byte]).expect("non ascii byte in input"),
                sequence,
                loc);
        }
        loc += 1;
    }
}
