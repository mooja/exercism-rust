#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub enum FrameError {
    PinOverflow,
    TooManyThrows,
}

pub enum FrameType {
    Open,
    Spare,
    Strike,
    Undecided,
}

pub struct Frame {
    pub rolls: Vec<u16>,
    pub extra_rolls: Vec<u16>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            rolls: vec![],
            extra_rolls: vec![],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), FrameError> {
        use FrameError::*;
        if pins > 10 {
            return Err(PinOverflow);
        }

        let sum: u16 = self.rolls.iter().sum();
        if pins + sum > 10 {
            return Err(PinOverflow);
        }

        if self.rolls.len() > 1 {
            return Err(TooManyThrows);
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> u16 {
        match self.frame_type() {
            FrameType::Open => self.rolls.iter().sum::<u16>(),
            FrameType::Spare => {
                let mut acc = self.rolls.iter().sum::<u16>();
                acc += self.extra_rolls[0];
                acc
            }

            FrameType::Strike => {
                let mut acc = self.rolls.iter().sum::<u16>();
                acc += self.extra_rolls.iter().sum::<u16>();
                acc
            }

            _ => 0,
        }
    }

    pub fn pins_down(&self) -> u16 {
        self.rolls.iter().sum::<u16>()
    }

    pub fn is_spare(&self) -> bool {
        if self.rolls.len() != 2 {
            return false;
        }

        if self.rolls.iter().sum::<u16>() != 10 {
            return false;
        }

        true
    }

    pub fn is_strike(&self) -> bool {
        self.rolls.len() == 1 && self.rolls[0] == 10
    }

    pub fn finished_scoring(&self) -> bool {
        if self.is_spare() {
            return self.extra_rolls.len() == 1;
        }

        if self.is_strike() {
            return self.extra_rolls.len() == 2;
        }

        self.rolls.len() == 2
    }

    pub fn finished_rolls(&self) -> bool {
        self.rolls.len() == 2 || self.rolls[0] == 10
    }

    pub fn frame_type(&self) -> FrameType {
        match self.rolls.len() {
            0 => FrameType::Undecided,
            1 => {
                if self.rolls[0] == 10 {
                    return FrameType::Strike;
                }
                FrameType::Undecided
            }

            2 if self.pins_down() == 10 => FrameType::Spare,
            2 => FrameType::Open,
            _ => panic!("too many rolls in one frame"),
        }
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: vec![] }
    }

    pub fn is_finished(&self) -> bool {
        match self.frames.len() {
            0..=9 => false,

            _ => {
                for frame in &self.frames[..10] {
                    if !frame.finished_scoring() {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn get_next_rolls(&self, frame_idx: usize, nrolls: usize) -> Vec<u16> {
        let mut rv = vec![];
        let mut frame_idx = frame_idx + 1;
        while rv.len() < nrolls {
            match self.frames.get(frame_idx) {
                None => break,
                Some(ref frame) => {
                    for roll in frame.rolls.iter() {
                        rv.push(roll.clone());
                        if rv.len() == nrolls {
                            break;
                        }
                    }
                    frame_idx += 1;
                }
            }
        }
        rv
    }

    fn compute_extra_rolls(&mut self) {
        let mut extra_rolls: Vec<Vec<u16>> = vec![];
        for (idx, frame) in self.frames.iter().enumerate() {
            let nrolls: usize;
            match frame.frame_type() {
                FrameType::Spare => nrolls = 1,
                FrameType::Strike => nrolls = 2,
                _ => nrolls = 0,
            }
            extra_rolls.push(self.get_next_rolls(idx, nrolls));
        }

        for (idx, frame) in self.frames.iter_mut().enumerate() {
            frame.extra_rolls = extra_rolls[idx].clone();
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_finished() {
            return Err(Error::GameComplete);
        }

        if self.frames.is_empty() || self.frames.last().unwrap().finished_rolls() {
            self.frames.push(Frame::new());
        }

        let last_frame: &mut Frame = self.frames.last_mut().unwrap();
        match last_frame.roll(pins) {
            Ok(()) => {
                self.compute_extra_rolls();
                Ok(())
            }

            Err(FrameError::PinOverflow) => Err(Error::NotEnoughPinsLeft),
            Err(FrameError::TooManyThrows) => {
                let mut frame = Frame::new();
                let _ = frame.roll(pins);
                self.frames.push(frame);
                self.compute_extra_rolls();
                Ok(())
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_finished() {
            return None;
        }

        let mut total = 0;
        for frame in &self.frames[..10] {
            total += frame.score();
        }
        Some(total)
    }
}
