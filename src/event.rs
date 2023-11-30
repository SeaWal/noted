use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent, MouseEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy)]
pub enum EventType {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::Sender<EventType>,

    #[allow(dead_code)]
    receiver: mpsc::Receiver<EventType>,

    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);

        let (sender, receiver) = mpsc::channel();

        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            Event::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(EventType::Key(e))
                                } else {
                                    Ok(())
                                }
                            }

                            Event::Mouse(e) => sender.send(EventType::Mouse(e)),

                            Event::Resize(w, h) => sender.send(EventType::Resize(w, h)),

                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender
                            .send(EventType::Tick)
                            .expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> Result<EventType> {
        Ok(self.receiver.recv()?)
    }
}
