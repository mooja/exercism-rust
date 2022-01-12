#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() == b.len() && a == b {
        return Comparison::Equal
    } else if a.len() == b.len() && a != b {
        return Comparison::Unequal
    } else if a.len() == 0 {
        return Comparison::Sublist
    } else if b.len() == 0 {
        return Comparison::Superlist
    } 

    let sm_list = if a.len() < b.len() { a } else { b };
    let lg_list = if a.len() > b.len() { a } else { b };
    let mut lg_contains_sm = false;
    for sublist in lg_list.windows(sm_list.len()) {
        if sublist == sm_list {
            lg_contains_sm = true;
            break;
        }
    }

    match lg_contains_sm {
        false => Comparison::Unequal,
        true => match sm_list == a {
            true => Comparison::Sublist,
            false => Comparison::Superlist
        }
    }
}
