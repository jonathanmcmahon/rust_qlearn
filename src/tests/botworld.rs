/// A testing framework for `QLearner` agents.
///
/// # Examples
///
/// ```
/// use tests::botworld::test_learner;
///
/// test_learner()
/// ```
use QLearner;
#[macro_use]
use macros;
use rand::distributions::IndependentSample;

/// Find and return the location of a unique object.
fn get_object_location(worldmap: &Vec<Vec<usize>>, obj: usize) -> (usize, usize) {
    let mut row : usize = 0;
    let mut col : usize = 0;
    for r in 0..worldmap.len() {
        for c in 0..worldmap[0].len() {
            if worldmap[r][c] == obj {
                col = c;
                row = r;
            }
        }
    }
    if (row + col) < 0 {
        panic!("Object is in an undefined location.");
    }
    (row, col)
}

/// Move bot and get reward
fn move_bot(worldmap: &mut Vec<Vec<usize>>, prev_pos: (usize, usize), action: usize) -> ((usize, usize), f32) {
    let (old_row, old_col) = prev_pos;
    let mut new_row = old_row as isize;
    let mut new_col = old_col as isize;

    let quicksand_penalty = -100.0;

    if action == 0 {             // north
        new_row -= 1;
    } else if action == 1 {      // east
        new_col += 1;
    } else if action == 2 {      // south
        new_row += 1;
    } else if action == 3 {      // west
        new_col -= 1;
    }

    /// Default negative reward
    let mut reward = -1.0;

    debug!("worldmap.len(): {}  worldmap[0].len(): {}", worldmap.len(), worldmap[0].len());
    // Detect if outside map
    if new_row < 0 { 
        new_row = old_row as isize;
        new_col = old_col as isize;
    } else if new_row >= worldmap.len() as isize { 
        new_row = old_row as isize;
        new_col = old_col as isize;
    } else if new_col < 0 { 
        new_row = old_row as isize;
        new_col = old_col as isize;
    } else if new_col >= worldmap[0].len() as isize { 
        new_row = old_row as isize;
        new_col = old_col as isize;
    // Detect if obstacle
    } else if worldmap[new_row as usize][new_col as usize] == 1 {
        new_row = old_row as isize;
        new_col = old_col as isize;
    // Detect quicksand
    } else if worldmap[new_row as usize][new_col as usize] == 5 {
        reward = quicksand_penalty;
        worldmap[new_row as usize][new_col as usize] = 6;
    } else if worldmap[new_row as usize][new_col as usize] == 6 {
        reward = quicksand_penalty;
        worldmap[new_row as usize][new_col as usize] = 6; 
    // Detect goal state
    } else if worldmap[new_row as usize][new_col as usize] == 3 {
        reward = 1.0;
    }
    debug!("Post-check new_row, new_col: ({},{})", new_row, new_col);
    ((new_row as usize, new_col as usize), reward)
}

/// Convert location tuple to an integer
fn discretize(pos: &(usize, usize)) -> usize {
    debug!("discretize(): pos.0, pos.1: {},{}", pos.0, pos.1);
    (pos.0 * 10) + pos.1
}

fn test (mut worldmap: Vec<Vec<usize>>, iterations: usize, mut learner: QLearner, mut rng: ::rand::ThreadRng) -> f32 {
    let bot_marker = 3;
    let goal_marker = 7;

    let ractions = ::Range::new(0, &learner.n_actions-1);
    let prob = ::Range::new(0.0, 1.0);
    let random_rate = 0.20;

    let max_moves = 100_000;

    let start_pos = get_object_location(&worldmap, bot_marker);
    info!("Start location: {:?}", start_pos);
    let goal_pos = get_object_location(&worldmap, goal_marker);
    info!("Goal location: {:?}", goal_pos);
    let mut scores = vec![0.0; iterations as usize];
    for i in 1..iterations+1 { 
        let mut total_reward = 0.0;
        let mut localmap = worldmap.clone();
        let mut bot_pos = start_pos;
        let mut state = discretize(&bot_pos);
        let mut action = learner.query(state, state, 0, 0.0, true);
        let mut count = 0;
        while (bot_pos != goal_pos) && (count< max_moves) {
            let (new_pos, reward) = move_bot(&mut worldmap, bot_pos, action);
            let r = reward;
            if new_pos == goal_pos {
                let r = 1.0;
            }

            state = discretize(&new_pos);
            action = learner.query(state, discretize(&bot_pos), action, r, true);

            // Stochastic environment -- some randomness in action execution
            let rprob = prob.ind_sample(&mut rng);
            if rprob < random_rate {
                action = ractions.ind_sample(&mut rng);
                debug!("Movement error: action {}.", action);
            }

            debug!("bot_pos: ({:?}),  new_pos: ({:?})", bot_pos, new_pos);
            if worldmap[bot_pos.0 as usize][bot_pos.1 as usize] != 6 {
                worldmap[bot_pos.0 as usize][bot_pos.1 as usize] = 4;
            }
            if worldmap[new_pos.0 as usize][new_pos.1 as usize] != 6 {
                worldmap[new_pos.0 as usize][new_pos.1 as usize] = 2;
            }
            bot_pos = new_pos;
            total_reward += reward;
            count += 1;
        }
        scores[i-1] = total_reward;
    }
    /// Calculate average score
    let mut avg_score = 0.0;
    for i in scores.iter() {
        debug!("iteration score: {}", i);
        avg_score += *i;
    }
    avg_score / scores.len() as f32 
}

/// Test a QLearner using BotWorld
pub fn test_learner() {

    println!("#### RUNNING QLEARNER TEST ####");

    let worldmap = vec![[0, 1, 0, 0, 0, 1, 0, 1, 0, 7].to_vec(), 
                        [1, 0, 0, 0, 0, 0, 1, 0, 0, 0].to_vec(), 
                        [0, 0, 1, 1, 0, 1, 0, 1, 0, 1].to_vec(), 
                        [1, 1, 0, 0, 0, 0, 1, 0, 0, 0].to_vec(), 
                        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0].to_vec(), 
                        [0, 1, 0, 1, 1, 0, 0, 1, 0, 0].to_vec(), 
                        [1, 0, 0, 0, 0, 1, 0, 0, 0, 1].to_vec(), 
                        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0].to_vec(), 
                        [1, 0, 1, 0, 0, 1, 0, 0, 1, 0].to_vec(), 
                        [3, 0, 0, 0, 0, 0, 1, 0, 0, 0].to_vec()];

    debug!("World Map: \n{:?}", worldmap);

    let original_worldmap = worldmap.clone();
    let mut rng = ::rand::thread_rng();

    // Train learner
    let mut learner = QLearner::new(120, 4);
    let iterations = 5000;
    let total_reward = test(worldmap, iterations, learner, rng);
    info!("{} iterations, avg reward: {}", iterations, total_reward);
    println!("#### QLEARNER TEST COMPLETE ####");

}