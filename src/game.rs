#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use piston_window::*;
use rand::random;
use std::collections::HashSet;

const PLAYER_COLOR: [f32; 4] = [0.0, 1, 0.0, 1.0];
const ENEMY_COLOR: [f32; 4] = [0.0, 1, 0.0, 1.0];
const BULLET_COLOR: [f32; 4] = [0.0, 1, 0.0, 1.0];
