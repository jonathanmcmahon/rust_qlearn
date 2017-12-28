# rust_qlearn #

## A rust implementation of the Q-Learning algorithm ##

This implementation requires a finite, discrete state space. 

In the botworld example provided, the 2D botworld state space is converted into a 1D state space for the `QLearner` using the *discretize()* function. 

To run an example training session, clone this repository, `cd` into it, and run the following:

```bash
export RUST_LOG_LEVEL=4
cargo test -- --nocapture
```

You will see output like the following indicated a successful test:

```
#### RUNNING QLEARNER TEST ####
Start location: (9, 0)
Goal location: (0, 9)
5000 iterations, avg reward: -49.0656
#### QLEARNER TEST COMPLETE ####
```