fn convert_char(v: u64, c: char) -> u64 {
    let code: u64 = c.into();
    ((v + code) * 17) % 256
}

fn hash(part: &str) -> u64 {
    part.chars().fold(0, |ascii, c| convert_char(ascii, c))
}

pub(crate) fn eval_file(file: &str) -> u64 {
    file.split(',')
        .filter(|s| !s.is_empty())
        .fold(0, |acc, part| acc + hash(part))
}

pub(crate) fn eval_file_2(file: &str) -> usize {
    let mut boxes = (0..256)
        .map(|_| Vec::<(String, u32)>::new())
        .collect::<Vec<_>>();
    file.split(',').filter(|s| !s.is_empty()).for_each(|v| {
        let mut label = String::new();
        let mut chars = v.chars();
        let mut idx = 0;
        while let Some(c) = chars.next() {
            if c.is_alphabetic() {
                label.push(c);
                idx = convert_char(idx, c);
            } else {
                let b = boxes.get_mut(idx as usize).expect("Is present");
                let idx = b
                    .iter()
                    .enumerate()
                    .find_map(|(i, (v, _))| if v == &label { Some(i) } else { None });
                match c {
                    '-' => {
                        if let Some(idx) = idx {
                            b.remove(idx);
                        }
                        return;
                    }
                    '=' => {
                        let mut nbr = 0;
                        while let Some(v) = chars.next() {
                            nbr = nbr * 10 + v.to_digit(10).expect("Should be nbr");
                        }
                        if let Some(idx) = idx {
                            b[idx].1 = nbr;
                        } else {
                            b.push((label, nbr));
                        }
                        // match map.entry(label) {
                        //     Entry::Occupied(mut occ) => occ.get_mut().0 = nbr,
                        //     Entry::Vacant(v) => {
                        //         v.insert((nbr, len));
                        //     }
                        // }
                        // map.entry(label)
                        //     .and_modify(|(l, _)| *l = nbr)
                        //     .or_insert((nbr, len));
                        return;
                    }
                    _ => unreachable!(),
                }
            }
        }
        unreachable!()
    });
    boxes.iter().enumerate().fold(0, |acc, (idx, entry)| {
        let res = entry
            .into_iter()
            .enumerate()
            .fold(0, |to_add, (i, (_, l))| {
                to_add + ((i + 1) * (idx + 1) * *l as usize)
            });
        acc + res
    })
}
pub(crate) fn print_sol_1(file: &str) {
    print!("res : {}", eval_file(file));
}
pub(crate) fn print_sol_2(file: &str) {
    print!("res : {}", eval_file_2(file));
}

#[cfg(test)]
mod tests {
    use super::{eval_file, eval_file_2};

    fn data() -> &'static str {
        r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#
    }
    #[test]
    fn test_0() {
        assert_eq!(1320, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(145, eval_file_2(data()));
    }
}
