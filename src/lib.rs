/// Constructs a new `QLearner`.
///
/// # Examples
///
/// ```
/// use qlearn::QLearner;
///
/// let (state, last_state, action, reward, update) = (99, 98, 1, 0.0, true);
///
/// let mut learner = QLearner::new(100, 5);
/// let next_action = learner.query(state, last_state, action, reward, update);
///
/// println!("QLearner selected action: {}", next_action);
/// ```
extern crate rand;

use rand::distributions::{IndependentSample, Range};

mod macros;

/// The `QLearner` type. See [the module level documentation](index.html) for more.
pub struct QLearner {
    /// Number of states
    pub n_states: usize,
    /// Number of actions
    pub n_actions: usize,
    /// Learning rate
    pub alpha: f32,
    /// Discount factor
    pub gamma: f32,
    /// Initial randomness 
    pub rar: f32,
    /// Randomness decay rate
    pub radr: f32,
    /// Table of Q values
    pub q_table: Vec<Vec<f32>>,
    /// Random number generator
    pub rng: rand::ThreadRng,
}


impl QLearner {

    /// Instantiate a new QLearner with some sensible defaults.
    pub fn new(n_states: usize, n_actions: usize) -> QLearner {
        let mut qt = vec![vec![0.0; n_actions]; n_states];
        let mut rng = rand::thread_rng();

        let uniform = Range::new(-1.0, 1.0);
        for i in 0..n_states {
            for j in 0..n_actions {
                    qt[i][j] = uniform.ind_sample(&mut rng);
            }
        }
        QLearner { n_states, n_actions, q_table: qt, rng: rng, alpha: 0.2, gamma: 0.9, rar: 0.5, radr: 0.99 }
    }

    /// This is the primary method for querying and training the learner. Use `update = true` when training.
    pub fn query(&mut self, s_curr: usize, s_prev: usize, action: usize, reward: f32, update: bool) -> usize {
        debug!("self.q_table[scurr]: {:?}", &self.q_table[s_curr]);
        let mut next_action = argmax(&self.q_table[s_curr]);
        if update == true {
            self.q_table[s_prev][action] = 
                (1.0 - self.alpha) 
                * self.q_table[s_prev][action] 
                + self.alpha 
                * (reward + self.gamma * self.q_table[s_curr][action])
        }
        // Inject some randomness into choice of action
        let prob = Range::new(0.0, 1.0);
        let rprob = prob.ind_sample(&mut self.rng);
        if rprob < self.rar {
            debug!("Random action ({} < {}).", rprob, self.rar);
            let ractions = Range::new(0, self.n_actions-1);
            next_action = ractions.ind_sample(&mut self.rng);
        }

        // Decay randomness
        if update == true {
            self.rar = self.rar * self.radr
        }
        next_action
    }
}


/// Find the argmax of a vector slice.
fn argmax(v: &Vec<f32>) -> usize {
    let mut max_val : f32 = std::f32::NEG_INFINITY; 
    let mut max_idx : usize = 0;
    debug!("v.len: {}", v.len());
    for i in 0..v.len() {
        if v[i] > max_val {
            max_val = v[i];
            max_idx = i;
        }
    }
    debug!("argmax: {}", max_idx);
    max_idx
}
    

#[cfg(test)]
mod tests {
    use super::*;
    mod botworld;

    #[test]
    fn qlearner_walk() {
        let mut q = QLearner::new(100, 4);
        let original_q_table = q.q_table.clone();
        let mut state = 0;
        let mut prev_state = 1;
        let mut action = 3;
        let reward = 1.0;
        for _ in 0..10000 {
            action = q.query(state, prev_state, action, reward, true);
            prev_state = state;
            state = (state + 3) % 100
        }
        // Compare "trained" q-table to original
        for i in 0..q.n_states {
            for j in 0..q.n_actions {
                if original_q_table[i][j] != q.q_table[i][j] {
                    debug!("Different value at ({},{})", i, j);
                }
            }
        }
    }

    #[test]
    fn qlearner_test() {
        use tests::botworld::test_learner;
        test_learner()
    }
}
