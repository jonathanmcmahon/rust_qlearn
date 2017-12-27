/// Constructs a new `QLearner`.
///
/// # Examples
///
/// ```
/// use qlearn::QLearner;
///
/// let learner = QLearner::new(100, 5);
/// ```
extern crate rand;

use rand::distributions::{IndependentSample, Range};


/// The `QLearner` type. See [the module level documentation](index.html) for more.
pub struct QLearner {
    pub n_states: usize,
    pub n_actions: usize,
    pub alpha: f32,
    pub gamma: f32,
    pub rar: f32,
    pub radr: f32,
    pub q_table: Vec<Vec<f32>>,
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
        let mut next_action = argmax(&self.q_table[s_curr]);
        if update == true {
            self.q_table[s_prev][action] = (1.0 - self.alpha) * self.q_table[s_prev][action] + self.alpha * (reward + self.gamma * self.q_table[s_curr][action])
        }
        // Inject some randomness into choice of action
        let prob = Range::new(0.0, 1.0);
        let rprob = prob.ind_sample(&mut self.rng);
        if rprob < self.rar {
            let ractions = Range::new(0, self.n_actions);
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
    for i in 0..v.len() {
        if v[i] > max_val {
            max_val = v[i];
            max_idx = i;
        }
    }
    max_idx
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qlearner_walk() {
        let mut q = QLearner::new(100, 5);
        let original_q_table = q.q_table.clone();
        let mut state = 0;
        let mut prev_state = 1;
        let mut action = 4;
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
                    println!("Different value at ({},{})", i, j)
                }
            }
        }
    }
}
