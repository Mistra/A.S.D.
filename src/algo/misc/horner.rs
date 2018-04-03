pub fn horner(variables: &mut[isize], x: isize) -> isize {
    let mut core = variables[0];
    let var = &mut variables[1 ..];
    for elem in var.iter() {
        core = core * x + elem;
    }
    core
}

pub fn power(x: isize, pow: usize) -> isize {
    let mut result = 1;
    for _ in 0 .. pow {
        result *= x;
    }
    result
}

pub fn polynome(variables: &mut[isize], x: isize) -> isize {
    let mut result = 0;
    for (i, elem) in variables.iter().rev().enumerate() {
        let x = power(x, i);
        result += elem * x;
    }
    result
}

#[cfg(test)]
mod test {
    use super::polynome;
    use super::horner;
    #[test]
    fn basic() {
        let funcs = [polynome, horner];
        for poly in funcs.iter() {
            let result = poly(&mut [2], 2);
            assert!(result == 2);

            let result = poly(&mut [3, 2], 2);
            assert!(result == 8);

            let result = poly(&mut [4, 3, 2], 3);
            assert!(result == 47);
        }
    }
}
