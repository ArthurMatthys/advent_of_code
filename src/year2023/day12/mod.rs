use std::collections::HashMap;

fn solve(
    springs: Vec<char>,
    matching_springs: Vec<usize>,
    memo: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    if (springs.is_empty() || springs.iter().all(|c| c == &'.' || c == &'?'))
        && matching_springs.is_empty()
    {
        return 1;
    } else if springs.is_empty() && !matching_springs.is_empty() {
        return 0;
    } else if !springs.is_empty() && matching_springs.is_empty() {
        return 0;
    }

    if let Some(v) = memo.get(&(springs.clone(), matching_springs.clone())) {
        return *v;
    }

    let mut tot = 0;

    if ['#', '?'].contains(&springs[0]) {
        let to_check = matching_springs[0];
        if springs.len() >= to_check
            && (1..to_check).all(|idx| springs[idx] == '#' || springs[idx] == '?')
            && (springs.len() == to_check || springs[to_check] != '#')
        {
            let cpy = if springs.len() <= to_check + 1 {
                vec![]
            } else {
                springs[to_check + 1..].to_vec()
            };
            let nbrs = if matching_springs.len() == 1 {
                vec![]
            } else {
                matching_springs[1..].to_vec()
            };

            tot += solve(cpy, nbrs, memo);
        }
        if true {}
    }

    if ['.', '?'].contains(&springs[0]) {
        let cpy = springs[1..].to_vec();
        tot += solve(cpy, matching_springs.clone(), memo);
    }
    memo.insert((springs, matching_springs), tot);

    tot
}

pub(crate) fn eval_file(file: &str) -> usize {
    let mut memo = HashMap::new();
    file.lines().filter(|l| !l.is_empty()).fold(0, |acc, l| {
        let mut line = l.split(' ');
        let springs = line
            .next()
            .expect("Should have springs")
            .chars()
            .collect::<Vec<_>>();
        let nbrs = line
            .next()
            .expect("Should have count")
            .split(',')
            .map(|c| c.parse::<usize>().expect("Should be nbr"))
            .collect::<Vec<_>>();
        let ret = solve(springs, nbrs, &mut memo);
        acc + ret
    })
}

pub(crate) fn eval_file_2(file: &str) -> usize {
    let mut memo = HashMap::new();
    file.lines().filter(|l| !l.is_empty()).fold(0, |acc, l| {
        let mut line = l.split(' ');
        let springs = line
            .next()
            .expect("Should have springs")
            .chars()
            .collect::<Vec<_>>();
        let nbrs = line
            .next()
            .expect("Should have count")
            .split(',')
            .map(|c| c.parse::<usize>().expect("Should be nbr"))
            .collect::<Vec<_>>();

        let mut tot_springs = vec![];
        let mut tot_nbrs = vec![];

        (0..5).for_each(|i| {
            springs.iter().for_each(|v| tot_springs.push(*v));
            if i != 4 {
                tot_springs.push('?')
            };
            nbrs.iter().for_each(|v| tot_nbrs.push(*v));
        });
        // eprintln!("springs : {springs:?} // nbrs : {nbrs:?}");
        let ret = solve(tot_springs, tot_nbrs, &mut memo);
        // eprintln!("ret : {ret}");
        acc + ret
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
        r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#
    }
    #[test]
    fn test_0() {
        assert_eq!(21, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(525152, eval_file_2(data()));
    }
}
