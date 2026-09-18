use serde::{Deserialize, Serialize};

pub const MAX_STAT: u8 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mood { Idle, Happy, Coding, Sleeping, Sick }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action { Feed, Coffee, Rest, Play, Code, Medicine }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PetState {
    pub name: String,
    pub age_minutes: u32,
    pub health: u8,
    pub energy: u8,
    pub happiness: u8,
    pub focus: u8,
    pub fullness: u8,
    pub xp: u32,
    pub level: u16,
    pub care_mistakes: u16,
    pub projects: u16,
    pub bugs_fixed: u16,
    pub mood: Mood,
}

impl Default for PetState {
    fn default() -> Self {
        Self { name:"Byte".into(), age_minutes:0, health:100, energy:80, happiness:80,
            focus:60, fullness:75, xp:0, level:1, care_mistakes:0, projects:0,
            bugs_fixed:0, mood:Mood::Idle }
    }
}

fn add(v: u8, n: u8) -> u8 { v.saturating_add(n).min(MAX_STAT) }
fn sub(v: u8, n: u8) -> u8 { v.saturating_sub(n) }

impl PetState {
    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Feed => { self.fullness=add(self.fullness,25); self.happiness=add(self.happiness,4); self.mood=Mood::Happy; }
            Action::Coffee => { self.focus=add(self.focus,22); self.energy=add(self.energy,8); self.health=sub(self.health,2); self.mood=Mood::Happy; }
            Action::Rest => { self.energy=add(self.energy,35); self.focus=add(self.focus,8); self.mood=Mood::Sleeping; }
            Action::Play => { self.happiness=add(self.happiness,18); self.energy=sub(self.energy,10); self.xp+=5; self.mood=Mood::Happy; }
            Action::Code => { self.focus=sub(self.focus,12); self.energy=sub(self.energy,14); self.fullness=sub(self.fullness,5); self.xp+=20; self.projects+=1; self.mood=Mood::Coding; }
            Action::Medicine => { self.health=add(self.health,30); self.mood=Mood::Idle; }
        }
        self.level = 1 + (self.xp / 100) as u16;
    }

    pub fn advance_minutes(&mut self, minutes: u32) {
        for _ in 0..minutes {
            self.age_minutes=self.age_minutes.saturating_add(1);
            if self.age_minutes % 20 == 0 { self.fullness=sub(self.fullness,1); }
            if self.age_minutes % 25 == 0 { self.energy=sub(self.energy,1); }
            if self.age_minutes % 30 == 0 { self.focus=sub(self.focus,1); }
            if self.fullness == 0 && self.age_minutes % 30 == 0 {
                self.health=sub(self.health,2); self.care_mistakes=self.care_mistakes.saturating_add(1);
            }
            if self.health < 25 { self.mood=Mood::Sick; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn code_is_deterministic() {
        let mut a=PetState::default(); let mut b=a.clone();
        a.apply(Action::Code); b.apply(Action::Code);
        a.advance_minutes(120); b.advance_minutes(120);
        assert_eq!(a,b);
    }
    #[test] fn stats_are_bounded() {
        let mut p=PetState::default();
        for _ in 0..20 { p.apply(Action::Coffee); p.apply(Action::Feed); }
        assert!(p.health<=100 && p.energy<=100 && p.focus<=100 && p.fullness<=100);
    }
}
