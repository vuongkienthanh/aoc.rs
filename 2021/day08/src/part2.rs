use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut sum = 0;
    for (mut line, output) in input {
        line.extend_from_slice(&output[..]);
        let one = line.iter().find(|x| x.count_ones() == 2).unwrap();
        let seven = line.iter().find(|x| x.count_ones() == 3).unwrap();
        let a = seven ^ one;
        let four = line.iter().find(|x| x.count_ones() == 4).unwrap();
        let four_a = a | four;
        let nine = line
            .iter()
            .find(|x| x.count_ones() == 6 && (**x & four_a) == four_a)
            .unwrap();
        let g = nine ^ four_a;
        let eight = line.iter().find(|x| x.count_ones() == 7).unwrap();
        let e = eight ^ nine;
        let acefg = a | g | e | one;
        let zero = line
            .iter()
            .find(|x| x.count_ones() == 6 && (**x & acefg) == acefg)
            .unwrap();
        let b = zero ^ acefg;
        let d = eight ^ zero;
        let adeg = a | d | e | g;
        let two = line
            .iter()
            .find(|x| x.count_ones() == 5 && (**x & adeg) == adeg)
            .unwrap();
        let c = two ^ adeg;
        let f = one ^ c;

        let mut output_sum = 0;
        for num in output {
            output_sum *= 10;
            match num {
                x if x == *zero => (),
                x if x == *one => output_sum += 1,
                x if x == *two => output_sum += 2,
                x if x == a | c | d | f | g => output_sum += 3,
                x if x == *four => output_sum += 4,
                x if x == a | b | d | f | g => output_sum += 5,
                x if x == a | b | d | e | f | g => output_sum += 6,
                x if x == *seven => output_sum += 7,
                x if x == *eight => output_sum += 8,
                x if x == *nine => output_sum += 9,
                _ => panic!(),
            }
        }
        sum += output_sum;
    }
    sum
}
