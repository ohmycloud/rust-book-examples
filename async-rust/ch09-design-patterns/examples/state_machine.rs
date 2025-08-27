enum State {
    On,
    Off,
}

enum Event {
    SwitchOn,
    SwitchOff,
}

impl State {
    async fn transition(self, event: Event) -> Self {
        match (&self, event) {
            (State::On, Event::SwitchOff) => {
                println!("Transitioning to the Off state");
                State::Off
            }
            (State::Off, Event::SwitchOn) => {
                println!("Transitioning to the On state");
                State::On
            }
            _ => {
                println!("No transition possible, staying in the current state");
                self
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut state = State::On;

    state = state.transition(Event::SwitchOff).await;
    state = state.transition(Event::SwitchOn).await;
    state = state.transition(Event::SwitchOn).await;

    match state {
        State::On => println!("The state machine is in the On state"),
        _ => println!("The state machine is not in the expected state"),
    }
}
