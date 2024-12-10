enum Block {
    Empty(usize),       // has size
    File(usize, usize), // has size and value
}

fn parse(s: &str) -> Vec<Block> {
    let mut out = Vec::new();
    let mut chars = s.chars();
    let mut value = 0;

    while let Some(file_byte) = chars.next() {
        if let Some(block_size) = file_byte.to_digit(10) {
            out.push(Block::File(block_size as usize, value));
            value += 1;
        }

        if let Some(empty_byte) = chars.next() {
            if let Some(block_size) = empty_byte.to_digit(10) {
                out.push(Block::Empty(block_size as usize));
            }
        }
    }

    out
}

fn main() {
    let input = include_str!("input");
    let blocks = parse(input);
    let mut res = 0;

    let mut pos = 0;
    let mut back_i = blocks.len() - 1;
    let mut back_size = 0;
    let mut back_val = 0;
    if let Block::File(size, val) = blocks[back_i] {
        back_size = size;
        back_val = val;
    }

    for i in 0..blocks.len() {
        if i >= back_i {
            break;
        }

        let front = &blocks[i];
        if let Block::File(size, value) = front {
            res += value * (2 * pos + size - 1) * size / 2; // derived from sum(n) = n(n+1)/2
            pos += size;
        } else if let Block::Empty(size) = front {
            let mut to_fill = *size;
            while to_fill > 0 {
                let can_use = usize::min(to_fill, back_size);
                to_fill -= can_use;
                back_size -= can_use;
                res += back_val * (2 * pos + can_use - 1) * can_use / 2; // derived from sum(n) = n(n+1)/2
                pos += can_use;

                if back_size == 0 {
                    back_i -= 1;
                    if back_i <= i {
                        break;
                    }
                    if let Block::File(size, val) = blocks[back_i] {
                        back_size = size;
                        back_val = val;
                    }
                }
            }
        }
    }

    for _ in 0..back_size {
        res += pos * back_val;
        pos += 1;
    }
    println!("Part 1 answer: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let vec = parse("10000000000000000001110111111111111");
        for block in vec.iter() {
            match block {
                Block::File(size, val) => println!("{val}: {size}"),
                Block::Empty(size) => println!("Empty: {size}"),
            }
        }
    }
}
