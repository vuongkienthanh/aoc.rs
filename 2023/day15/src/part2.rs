use std::collections::HashMap;

struct Lens <'a>{
    label: &'a str,
    focus: usize,
}

impl<'a> Lens<'a> {
    fn new(label: &str, focus: usize) -> Self {
        if focus<1 || focus >9 {
            panic!("focus out of range");
        }
        Self {
            label,
            focus,
        }
    }
}

pub fn process(_input: &str) -> usize {
    let input = _input.replace("\n", "");
    let mut boxes : HashMap<usize, Vec<Lens>> = HashMap::new();
    // for step in input.split(',') {
    //     match step.chars().last().unwrap() {
    //         '-' => todo!("dash"),
    //         equal_sign_step => todo!("equal"),
    //     }
    // }

    boxes.iter()
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
