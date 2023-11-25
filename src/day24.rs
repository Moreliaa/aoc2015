use itertools::Itertools;

pub fn run(input: String) {
    let packages = input.lines().map(|l| l.parse::<i32>().unwrap()).collect_vec();
    println!("Day24 Pt1: {}", pt1(&packages));
    println!("Day24 Pt2: {}", pt2(&input));
}

fn pt1(packages: &Vec<i32>) -> u128 {
    let total_weight: i32 = packages.iter().sum();
    let weight_per_compartment = total_weight / 3;
    println!("Total Wgt: {total_weight} Weight per Cmp: {weight_per_compartment}");
    // from input: at *least* 5 packages in pass, at *most* 6
    // possible distributions are 6,7,7 / 6,6,8 / 5,7,8 / 5,6,9 / 5,5,10
    // couldn't find anything for 5 -> 291 for 6
    // possible distributions are 6,7,7 / 6,6,8
    let mut possible_6:Vec<Vec<usize>> = vec![];
    for i in 0..packages.len() {
        for j in i + 1..packages.len() {
            for k in j + 1..packages.len() {
                for l in k + 1..packages.len() {
                    for m in l + 1..packages.len() {
                        for n in m + 1..packages.len() {
                            if packages[i] + packages[j] + packages[k] + packages[l] + packages[m] + packages[n] == weight_per_compartment { // weight
                                possible_6.push(vec![i,j,k,l,m,n]);
                            }
                        }
                    }
                }
            }
        }
    }

    // 0 possible solutions for len 7 -> shape is 6,6,8
    let mut possible_7:Vec<Vec<usize>> = vec![];
    for i in 0..packages.len() {
        for j in i + 1..packages.len() {
            for k in j + 1..packages.len() {
                for l in k + 1..packages.len() {
                    for m in l + 1..packages.len() {
                        for n in m + 1..packages.len() {
                            for o in n + 1..packages.len() {
                                if packages[i] + packages[j] + packages[k] + packages[l] + packages[m] + packages[n] + packages[o] == weight_per_compartment { // weight
                                    possible_7.push(vec![i,j,k,l,m,n,o]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    let mut solutions:Vec<(Vec<usize>, Vec<usize>)> = vec![];
    for i in 0..possible_6.len() {
            let p1 = &possible_6[i];
            solutions.push((p1.clone(), p1.clone()));
    }

    let mut min_qe: Option<u128> = None;
    let mut best_solution: Option<Vec<usize>> = None;
    for s in solutions.iter() {
        let pairs = [&s.0, &s.1];
        for p in pairs {
            if p.len() > 6 {
                continue;
            }
            let qe: u128 = p.iter().fold(1, |acc, idx| acc * packages[*idx] as u128);
            best_solution = match min_qe {
                Some(val) => if qe < val {Some(p.clone())} else {best_solution},
                None => Some(p.clone())
            };
            
            min_qe = match min_qe {
                Some(val) => Some(val.min(qe)),
                None => Some(qe)
            };

            
        }
    }

    println!("Len 6s: {} Len 7s: {}, Solutions: {}, Len packages: {}", possible_6.len(), possible_7.len(), solutions.len(), packages.len());
    println!("Best solution: {:#?}", best_solution.unwrap().iter().map(|item| packages[*item]).collect::<Vec<_>>() );
    min_qe.unwrap()
}

fn pt2(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {

    }
}