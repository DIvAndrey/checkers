use crate::bot::NegaScoutBot;

pub struct EvaluationBar {
    pub bot: NegaScoutBot,
    pub new_evaluation: f32,
    pub displayed_evaluation: f32,
    pub last_evaluated_move: i32,
}

impl Default for EvaluationBar {
    fn default() -> Self {
        EvaluationBar {
            bot: NegaScoutBot::new(1),
            new_evaluation: 0.0,
            displayed_evaluation: 0.0,
            last_evaluated_move: 0,
        }
    }
}