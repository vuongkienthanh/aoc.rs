use std::collections::HashMap;

struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

impl<'a> Lens<'a> {
    fn new(label: &'a str, focal_length: usize) -> Self {
        if focal_length < 1 || focal_length > 9 {
            panic!("focus out of range");
        }
        Self {
            label,
            focal_length,
        }
    }
}

fn hashlabel(label: &str) -> usize {
    label
        .chars()
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

pub fn process(_input: &str) -> usize {
    let input = _input.replace("\n", "");
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

    for step in input.split(',') {
        match step.chars().last().unwrap() {
            '-' => {
                let label = step.get(0..step.len() - 1).unwrap();
                let box_loc = hashlabel(label);
                let all_lens_in_box = boxes.entry(box_loc).or_default();
                let mut find_lens = None;
                for (i, l) in all_lens_in_box.into_iter().enumerate() {
                    if l.label == label {
                        find_lens = Some(i);
                        break;
                    }
                }
                if let Some(i) = find_lens {
                    all_lens_in_box.remove(i);
                }
            }
            _ => {
                let (label, focal_length) = step.split_once('=').unwrap();
                let lens = Lens::new(label, focal_length.parse().unwrap());
                let box_loc = hashlabel(label);
                let all_lens_in_box = boxes.entry(box_loc).or_default();
                let mut find_lens = None;
                for (i, l) in all_lens_in_box.into_iter().enumerate() {
                    if l.label == label {
                        find_lens = Some(i);
                        break;
                    }
                }
                if let Some(i) = find_lens {
                    *all_lens_in_box.get_mut(i).unwrap() = lens;
                } else {
                    all_lens_in_box.push(lens);
                }
            }
        }
    }

    boxes
        .iter()
        .map(|(k, v)| {
            let box_score = *k + 1;
            let all_lens_score = v
                .into_iter()
                .enumerate()
                .map(|(i, len)| {
                    let slot_score = i + 1;
                    let lens_score = len.focal_length;
                    slot_score * lens_score
                })
                .sum::<usize>();
            box_score * all_lens_score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(process(input), 145);
    }
}
