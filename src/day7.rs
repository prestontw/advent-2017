use nom::types::CompleteStr;
use nom::{alpha, digit, space};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug)]
struct Program {
    name: String,
    weight: usize,
    children: Option<Vec<String>>,
}

fn build_platter<'a, I>(ps: I) -> HashMap<String, &'a Program>
where
    I: IntoIterator<Item = &'a Program>,
{
    ps.into_iter().fold(HashMap::new(), |mut acc, cur| {
        acc.insert(cur.name.clone(), cur);
        acc
    })
}

fn total_weight_or_uneven_child(weights: &[usize], node_weights: &[usize]) -> Result<usize, usize> {
    let weight_counts =
        weights
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<usize, usize>, cur| {
                {
                    let count = acc.entry(*cur).or_insert(0);
                    *count += 1;
                }
                acc
            });
    // check if have multiple ones
    // TODO: really, check to make sure that length is only two
    if weight_counts.len() > 2 {
        panic!("why do we have more than two weights? {:?}", weight_counts);
    }
    // and return the difference between the two, or the difference
    // between the weight of the children of the bad node
    // and the weight the other subtrees have
    let singulars: Vec<usize> = weight_counts
        .iter()
        .filter(|(_k, &v)| v == 1)
        .map(|(k, _v)| *k)
        .collect();

    if singulars.len() == 1 {
        let target = singulars[0];
        let other_weight = weight_counts
            .iter()
            .filter(|(&k, _v)| k != target)
            .map(|(k, _v)| *k)
            .nth(0)
            .unwrap();
        for (&weight, &node_weight) in weights.iter().zip(node_weights) {
            if weight == target {
                let diff = (other_weight as isize - target as isize) as isize;
                return Err((node_weight as isize + diff) as usize);
            }
        }
        panic!("should have found element by now");
    } else if singulars.len() > 1 {
        panic!(
            "ambiguous weights, could change more than one! {:?}",
            weights
        );
    }
    // if don't, return sum of weights
    else {
        Ok(weights.iter().sum())
    }
}

// return an either?
pub fn balanced_weight(i: &str) -> usize {
    let root = bottom_program(i);
    let programs = parse_lines(i);
    // TODO build tree from programs
    let tree: HashMap<String, &Program> = build_platter(programs.iter());

    fn children_weight(n: &str, tree: &HashMap<String, &Program>) -> Result<usize, usize> {
        let n = &tree[n];
        match n.children {
            None => Ok(n.weight),
            Some(ref children) => {
                // TODO check kids, if any are err, return that
                let ws: Vec<Result<usize, usize>> = children
                    .iter()
                    .map(|s: &String| children_weight(s, tree))
                    .collect();
                let uneven: Vec<Result<usize, usize>> = ws
                    .iter()
                    .filter(|r: &&Result<_, _>| r.is_err())
                    .cloned()
                    .collect();
                if uneven.len() == 1 {
                    uneven[0]
                } else if uneven.len() >= 1 {
                    panic!(
                        "more than one child's platter is unbalanced! {:?}",
                        children
                    );
                } else {
                    // if any weight is unequal of children, return that
                    let children_weight: Vec<usize> = ws.iter().map(|s| s.unwrap()).collect();
                    let individual_child_weights: Vec<usize> =
                        children.iter().map(|c| tree[c].weight).collect();
                    println!("{}: {:?}", n.name, children_weight);
                    let children_result = total_weight_or_uneven_child(
                        &children_weight[..],
                        &individual_child_weights[..],
                    );
                    // else, add current weight to total and return
                    match children_result {
                        Ok(children_sum) => Ok(children_sum + n.weight),
                        Err(u) => Err(u),
                    }
                }
            }
        }
    }
    // start at the bottom, check weights of children
    // if all children's children have balanced weight, check children themselves
    let res = children_weight(&root, &tree);
    match res {
        Ok(_) => panic!("not one program with bad weight"),
        Err(r) => r,
    }
}
#[test]
fn test_balanced_weight() {
    let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    assert_eq!(balanced_weight(input), 60_usize);

    /* let simpleinput = "r (77) -> a b c
    a (6)
    b (6)
    c (7)";
      assert_eq!(balanced_weight(simpleinput), 6);

      let diffinput = "r (77) -> a b c
    a (5) -> m n o
    b (7)
    m (1)
    n (1)
    o (1)
    c (1) -> h g
    h (3)
    g (3)";
      assert_eq!(balanced_weight(diffinput), 4);*/
}

pub fn bottom_program(i: &str) -> String {
    let programs = parse_lines(i);
    let (parents, _kids) = programs.iter().fold(
        (HashSet::<String>::new(), HashSet::<String>::new()),
        |(parents, children), p: &Program| {
            if let Some(ref c) = p.children {
                // iterate over children and remove from parents
                let mut new_parents = c.iter().fold(parents, |mut acc, kid| {
                    acc.remove(kid);
                    acc
                });
                // check to see if add p's name to parents
                if !children.contains(&p.name) {
                    new_parents.insert(p.name.clone());
                }
                // and add children to children
                let new_children = c.iter().fold(children, |mut acc, kid| {
                    acc.insert(kid.clone());
                    acc
                });
                (new_parents, new_children)
            } else {
                (parents, children)
            }
        },
    );

    if parents.len() == 1 {
        parents.iter().nth(0).unwrap().to_string()
    } else {
        panic!("more than one bottom program!")
    }
}
#[test]
fn example_bottom_program() {
    let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    assert_eq!(bottom_program(input), "tknk");
}

named!(weight<CompleteStr, usize>, delimited!(char!('('), number, char!(')')));

#[test]
fn test_weight() {
    assert_eq!(weight(CompleteStr("(77)")), Ok((CompleteStr(""), 77_usize)));
}

use std::str::FromStr;

named!(
  number<CompleteStr, usize>,
  map_res!(recognize!(digit),
  |cs: CompleteStr| usize::from_str(&cs))
);

named!(arrow<CompleteStr, CompleteStr>, tag!("->"));

// use opt! for parsing children
// opt!(arrow, space, many0(children), chars)

named!(
  children<CompleteStr, Vec<String>>,
  do_parse!(
    ws: separated_list!(tag!(", "), alpha)
      >> (ws
        .iter()
        .map(|c| String::from_str(c).unwrap())
        .collect())
  )
);

#[test]
fn test_children() {
    assert_eq!(
        children(CompleteStr("abcd, defg, als")),
        Ok((
            CompleteStr(""),
            vec!["abcd", "defg", "als"]
                .iter()
                .map(|c| c.to_string())
                .collect()
        ))
    );
}

named!(
  line<CompleteStr, Program>,
  do_parse!(
    n: alpha
      >> space
      >> w: weight
      >> c: opt!(do_parse!(space >> arrow >> space >> c: children >> (c)))
      >> (Program {
        name: String::from_str(&n).unwrap(),
        weight: w,
        children: c,
      })
  )
);

#[test]
fn test_lines() {
    assert_eq!(
        line(CompleteStr("pbga (66)")),
        Ok((
            CompleteStr(""),
            Program {
                name: "pbga".to_string(),
                weight: 66,
                children: None
            }
        ))
    );

    assert_eq!(
        line(CompleteStr("fwft (72) -> ktlj, cntj, xhth")),
        Ok((
            CompleteStr(""),
            Program {
                name: "fwft".to_string(),
                weight: 72,
                children: Some(
                    vec!["ktlj", "cntj", "xhth"]
                        .iter()
                        .map(|c| c.to_string())
                        .collect()
                )
            }
        ))
    );
}

fn parse_lines(i: &str) -> Vec<Program> {
    i.lines()
        .map(|l| {
            let res = line(CompleteStr(l));
            res.unwrap().1
        })
        .collect()
}
#[test]
fn test_parse_lines() {
    assert_eq!(
        parse_lines(
            "ktlj (57)
fwft (72) -> ktlj, cntj, xhth"
        ),
        vec![
            Program {
                name: "ktlj".to_string(),
                weight: 57,
                children: None,
            },
            Program {
                name: "fwft".to_string(),
                weight: 72,
                children: Some(
                    vec!["ktlj", "cntj", "xhth"]
                        .iter()
                        .map(|c| c.to_string())
                        .collect()
                )
            }
        ]
    )
}
