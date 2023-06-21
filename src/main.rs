use std::cmp;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, Ord, PartialEq)]
struct State {
    cannibals_left: i64,
    missionaries_left: i64,
    boat_left: bool,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(
            score(self.cannibals_left, self.missionaries_left)
                .cmp(&score(other.cannibals_left, other.missionaries_left))
                .reverse(),
        )
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cannibals_left.hash(state);
        self.missionaries_left.hash(state);
        self.boat_left.hash(state);
    }
}

trait StateQueue {
    fn push(&mut self, state: State);
    fn pop(&mut self) -> Option<State>;
    fn is_empty(&self) -> bool;
}

impl StateQueue for Vec<State> {
    fn push(&mut self, state: State) {
        self.push(state);
    }
    fn pop(&mut self) -> Option<State> {
        self.pop()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl StateQueue for BinaryHeap<State> {
    fn push(&mut self, state: State) {
        self.push(state);
    }
    fn pop(&mut self) -> Option<State> {
        self.pop()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

struct ValidateCannibalMissionaryBalanceProp {
    cannibals_left: i64,
    missionaries_left: i64,
    cannibals_right: i64,
    missionaries_right: i64,
    cannibals_boat: i64,
    missionaries_boat: i64,
}

fn validate_cannibal_missionary_balance(prop: ValidateCannibalMissionaryBalanceProp) -> bool {
    let left_side_balance =
        prop.cannibals_left <= prop.missionaries_left || prop.missionaries_left == 0;
    let right_side_balance =
        prop.cannibals_right <= prop.missionaries_right || prop.missionaries_right == 0;
    let boat_balance = prop.cannibals_boat <= prop.missionaries_boat || prop.missionaries_boat == 0;

    left_side_balance && right_side_balance && boat_balance
}

#[derive(Clone)]
struct BoatMovement {
    cannibals_boat: i64,
    missionaries_boat: i64,
    move_right: bool,
}

fn solve<T: Default + StateQueue>(
    cannibals_num: i64,
    missionaries_num: i64,
    boat_capacity: i64,
) -> Option<Vec<BoatMovement>> {
    let state = State {
        cannibals_left: cannibals_num,
        missionaries_left: missionaries_num,
        boat_left: true,
    };

    let mut history: HashMap<State, Vec<BoatMovement>> = HashMap::new();
    let mut queue = T::default();

    queue.push(state.clone());

    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        let cannibals_left = state.cannibals_left;
        let missionaries_left = state.missionaries_left;
        let cannibals_right = cannibals_num - cannibals_left;
        let missionaries_right = missionaries_num - missionaries_left;

        if cannibals_left == 0 && missionaries_left == 0 && !state.boat_left {
            return history.get(&state).map(|value| value.clone());
        }

        let max_cannibals_on_boat = if state.boat_left {
            cmp::min(boat_capacity, cannibals_left)
        } else {
            cmp::min(boat_capacity, cannibals_right)
        };

        let max_missionaries_on_boat = |cannibals_boat: i64| {
            if state.boat_left {
                cmp::min(boat_capacity - cannibals_boat, missionaries_left)
            } else {
                cmp::min(boat_capacity - cannibals_boat, missionaries_right)
            }
        };

        for cannibals_boat in 0..=max_cannibals_on_boat {
            for missionaries_boat in 0..=max_missionaries_on_boat(cannibals_boat) {
                if cannibals_boat + missionaries_boat == 0 {
                    continue;
                }

                fn update_counts(left: i64, right: i64, boat: i64, boat_left: bool) -> (i64, i64) {
                    if boat_left {
                        (left - boat, right + boat)
                    } else {
                        (left + boat, right - boat)
                    }
                }

                let (next_cannibals_left, next_cannibals_right) = update_counts(
                    cannibals_left,
                    cannibals_right,
                    cannibals_boat,
                    state.boat_left,
                );
                let (next_missionaries_left, next_missionaries_right) = update_counts(
                    missionaries_left,
                    missionaries_right,
                    missionaries_boat,
                    state.boat_left,
                );

                if !validate_cannibal_missionary_balance(ValidateCannibalMissionaryBalanceProp {
                    cannibals_left: next_cannibals_left,
                    missionaries_left: next_missionaries_left,
                    cannibals_right: next_cannibals_right,
                    missionaries_right: next_missionaries_right,
                    cannibals_boat,
                    missionaries_boat,
                }) {
                    continue;
                }

                let next_state = State {
                    cannibals_left: next_cannibals_left,
                    missionaries_left: next_missionaries_left,
                    boat_left: !state.boat_left,
                };

                if history.contains_key(&next_state) {
                    continue;
                }

                let mut next_history = match history.get(&state) {
                    Some(value) => value.clone(),
                    None => Vec::new(),
                };
                next_history.push(BoatMovement {
                    cannibals_boat,
                    missionaries_boat,
                    move_right: state.boat_left,
                });
                history.insert(next_state.clone(), next_history.clone());

                queue.push(next_state);
            }
        }
    }
    return None;
}

fn score(cannibals_left: i64, missionaries_left: i64) -> i64 {
    cannibals_left + missionaries_left
}

fn print_history(history: &Vec<BoatMovement>) {
    for action in history.iter() {
        println!("===========================================================");
        if action.move_right {
            println!(
                "(→) move right with {} 🧟 and {} 😇",
                action.cannibals_boat, action.missionaries_boat
            );
        } else {
            println!(
                "(←) move left with {} 🧟 and {} 😇",
                action.cannibals_boat, action.missionaries_boat
            );
        }
        println!("===========================================================");
        println!();
    }
}

fn main() {
    let cannibals = 10;
    let missionaries = 20;
    let boat_capacity = 3;

    let result_vec = solve::<Vec<State>>(cannibals, missionaries, boat_capacity);
    let result_heap = solve::<BinaryHeap<State>>(cannibals, missionaries, boat_capacity);
    match result_vec {
        Some(history) => {
            println!("Found solution! With Vec<State>");
            println!("===========================================================");
            println!("🧟 = cannibal");
            println!("😇 = missionary");
            println!("===========================================================");
            println!();
            println!("step counts: {}", history.len());
            print_history(&history);
        }
        None => {
            println!("===========================================================");
            println!("No solution found!");
            println!("===========================================================");
        }
    }
    match result_heap {
        Some(history) => {
            println!("Found solution! With BinaryHeap<State>");
            println!("===========================================================");
            println!("🧟 = cannibal");
            println!("😇 = missionary");
            println!("===========================================================");
            println!();
            println!("step counts: {}", history.len());
            print_history(&history);
        }
        None => {
            println!("===========================================================");
            println!("No solution found!");
            println!("===========================================================");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let cannibals = 3;
        let missionaries = 3;
        let boat_capacity = 2;

        let result_vec = solve::<Vec<State>>(cannibals, missionaries, boat_capacity);
        let result_heap = solve::<BinaryHeap<State>>(cannibals, missionaries, boat_capacity);

        assert!(result_vec.is_some());
        assert!(result_heap.is_some());
    }

    #[test]
    fn test_solve_no_solution() {
        let cannibals = 4;
        let missionaries = 3;
        let boat_capacity = 2;

        let result_vec = solve::<Vec<State>>(cannibals, missionaries, boat_capacity);
        let result_heap = solve::<BinaryHeap<State>>(cannibals, missionaries, boat_capacity);

        assert!(result_vec.is_none());
        assert!(result_heap.is_none());
    }

    #[test]
    fn test_validate_cannibal_missionary_balance() {
        let prop = ValidateCannibalMissionaryBalanceProp {
            cannibals_left: 1,
            missionaries_left: 2,
            cannibals_right: 1,
            missionaries_right: 2,
            cannibals_boat: 1,
            missionaries_boat: 1,
        };
        assert_eq!(validate_cannibal_missionary_balance(prop), true);

        let prop = ValidateCannibalMissionaryBalanceProp {
            cannibals_left: 2,
            missionaries_left: 1,
            cannibals_right: 2,
            missionaries_right: 1,
            cannibals_boat: 1,
            missionaries_boat: 1,
        };
        assert_eq!(validate_cannibal_missionary_balance(prop), false);
    }
}
