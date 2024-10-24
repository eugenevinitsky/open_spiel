extern crate libc;

use std::os::raw::c_void;
use libc::{c_char, free};
use std::slice;

mod open_spiel_bindings;
use open_spiel_bindings::*;

fn convert_and_free_cstring(c_buf: *mut c_char, len: u64) -> String {
  let bytes = unsafe { std::slice::from_raw_parts(c_buf as *const u8, len as usize) };
  let str_slice = unsafe { std::str::from_utf8_unchecked(bytes) };
  let str_buf: String = str_slice.to_owned();
  unsafe { free(c_buf as *mut c_void) };
  str_buf
}

pub struct State {
  state: *mut c_void,
}

pub struct Game {
  game: *mut c_void,
}

impl State {
  pub fn new(sptr: *mut c_void) -> State {
    State {
      state: sptr
    }
  }

  pub fn current_player(&self) -> i32 {
    unsafe { StateCurrentPlayer(self.state) as i32 }
  }

  pub fn clone(&self) -> State {
    unsafe { State { state: StateClone(self.state) } }
  }

  pub fn is_chance_node(&self) -> bool {
    let ret = unsafe { StateIsChanceNode(self.state) };
    ret == 1
  }

  pub fn is_terminal(&self) -> bool {
    let ret = unsafe { StateIsTerminal(self.state) };
    ret == 1
  }

  pub fn num_players(&self) -> usize {
    unsafe { StateNumPlayers(self.state) as usize }
  }

  pub fn returns(&self) -> Vec<f64> {
    let length = self.num_players();
    let mut returns_vec = Vec::with_capacity(length);
    unsafe {
      StateReturns(self.state, returns_vec.as_mut_ptr());
      returns_vec.set_len(length);
    }
    returns_vec
  }

  pub fn player_return(&self, player: i32) -> f64 {
    unsafe { StatePlayerReturn(self.state, player) }
  }

  pub fn legal_actions(&self) -> Vec<i64> {
    let mut c_num_legal_moves = 0;
    let c_buf = unsafe { StateLegalActions(self.state, &mut c_num_legal_moves) };
    unsafe {
      let vec = slice::from_raw_parts(c_buf, c_num_legal_moves as usize).to_vec();
      free(c_buf as *mut c_void);
      vec
    }
  }

  pub fn chance_outcomes(&self) -> Vec<(i64, f64)> {
    let legal_actions: Vec<i64> = self.legal_actions();
    let mut size = 0;
    let c_buf = unsafe { StateChanceOutcomeProbs(self.state, &mut size) };
    let length: usize = size as usize;
    let mut vec = vec![(0 as i64, 0.0 as f64); length];
    unsafe {
      let probs_slice = slice::from_raw_parts(c_buf, length);
      for i in 0..length {
        vec[i] = (legal_actions[i], probs_slice[i]);
      }
      free(c_buf as *mut c_void);
    }
    vec
  }

  pub fn apply_action(&self, action: i64) {
    unsafe { StateApplyAction(self.state, action) }
  }

  pub fn action_to_string(&self, player: i32, action: i64) -> String {
    let mut length = 0;
    let c_buf: *mut c_char = unsafe {
        StateActionToString(self.state, player, action, &mut length)
    };
    convert_and_free_cstring(c_buf, length)
  }

  pub fn to_string(&self) -> String {
    let mut length = 0;
    let c_buf: *mut c_char = unsafe { StateToString(self.state, &mut length) };
    convert_and_free_cstring(c_buf, length)
  }

  pub fn observation_string(&self) -> String {
    let mut length = 0;
    let c_buf: *mut c_char = unsafe {
        StateObservationString(self.state, &mut length)
    };
    convert_and_free_cstring(c_buf, length)
  }

  pub fn information_state_string(&self) -> String {
    let mut length = 0;
    let c_buf: *mut c_char = unsafe {
        StateInformationStateString(self.state, &mut length)
    };
    convert_and_free_cstring(c_buf, length)
  }

  pub fn observation_tensor(&self) -> Vec<f32> {
    let length = unsafe { StateObservationTensorSize(self.state) as usize};
    let mut obs_vec = Vec::with_capacity(length);
    unsafe {
      StateObservationTensor(self.state, obs_vec.as_mut_ptr(), length as i32);
      obs_vec.set_len(length);
    }
    obs_vec
  }

  pub fn information_state_tensor(&self) -> Vec<f32> {
    let length = unsafe { StateInformationStateTensorSize(self.state) as usize };
    let mut infostate_vec = Vec::with_capacity(length);
    unsafe {
      StateInformationStateTensor(self.state, infostate_vec.as_mut_ptr(), length as i32);
      infostate_vec.set_len(length);
    }
    infostate_vec
  }
}

impl Drop for State {
  fn drop(&mut self) {
     unsafe { DeleteState(self.state) }
  }
}

impl Game {
  pub fn new(game_name: String) -> Self {
    Game {
      game: unsafe { LoadGame(game_name.as_ptr() as *const i8) }
    }
  }

  pub fn short_name(&self) -> String {
    let mut length = 0;
    let c_buf = unsafe { GameShortName(self.game, &mut length) };
    convert_and_free_cstring(c_buf, length)
  }

  pub fn long_name(&self) -> String {
    let mut length = 0;
    let c_buf = unsafe { GameLongName(self.game, &mut length) };
    convert_and_free_cstring(c_buf, length)
  }

  pub fn new_initial_state(&self) -> State {
    unsafe { State::new(GameNewInitialState(self.game)) }
  }

  pub fn num_players(&self) -> i32 {
    unsafe { GameNumPlayers(self.game) as i32 }
  }

  pub fn max_game_length(&self) -> i32 {
    unsafe { GameMaxGameLength(self.game) as i32 }
  }

  pub fn num_distinct_actions(&self) -> i32 {
    unsafe { GameNumDistinctActions(self.game) as i32}
  }

  pub fn observation_shape(&self) -> Vec<i32> {
    let mut size = 0;
    let c_buf = unsafe { GameObservationTensorShape(self.game, &mut size) };
    unsafe {
      let vec = slice::from_raw_parts(c_buf, size as usize).to_vec();
      free(c_buf as *mut c_void);
      vec
    }
  }

  pub fn information_state_tensor_shape(&self) -> Vec<i32> {
    let mut size = 0;
    let c_buf = unsafe { GameInformationStateTensorShape(self.game, &mut size) };
    unsafe {
      let vec = slice::from_raw_parts(c_buf, size as usize).to_vec();
      free(c_buf as *mut c_void);
      vec
    }
  }
}

impl Drop for Game {
  fn drop(&mut self) {
     unsafe { DeleteGame(self.game) }
  }
}

