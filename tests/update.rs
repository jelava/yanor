use yanor_core::{
    Updatable,
    UpdateQueue,
    Message,
    Text,
    update::Effect
};

struct Dummy1 {
    update_count: u32,
    max_updates: u32
}

struct Dummy2 {
    has_updated: bool
}

impl Updatable for Dummy1 {
    fn update(&mut self, _effects: &mut Vec<Effect>) -> Option<u32> {
        self.update_count += 1;

        if self.update_count <= self.max_updates {
            Some(2)
        } else {
            None
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn deactivate(&mut self) {
        unimplemented!()
    }
}

impl Updatable for Dummy2 {
    fn update(&mut self, _effects: &mut Vec<Effect>) -> Option<u32> {
        if self.has_updated {
            None
        } else {
            self.has_updated = true;
            Some(25)
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn deactivate(&mut self) {
        unimplemented!()
    }
}

#[test]
fn ordering() {
    let mut queue = UpdateQueue::new();
    let mut dummy1 = Dummy1 { update_count: 0, max_updates: 10 };
    let mut dummy2 = Dummy2 { has_updated: false };

    queue.push(10, &mut dummy1);
    queue.push(0, &mut dummy2);

    let mut times = [0; 13];
    let mut i = 0;
    let mut effects = Vec::new();

    while let Some(time) = queue.update(&mut effects) {
        times[i] = time;
        i += 1;
    }

    assert_eq!(times, [0, 10, 12, 14, 16, 18, 20, 22, 24, 25, 26, 28, 30]);
}

struct Messenger {
    msg: Message
}

impl Messenger {
    fn new(name: &str, message: &str) -> Self {
        Messenger {
            msg: Message::normal(vec![
                Text::normal("Message from"),
                Text::bold(name),
                Text::normal(":"),
                Text::italic(message)
            ])
        }
    }
}

// text!(no!("Message from"), bf!(name), no!(":"), it!(message))

impl Updatable for Messenger {
    fn update(&mut self, effects: &mut Vec<Effect>) -> Option<u32> {
        effects.push(Effect::Log(self.msg.clone()));
        None
    }

    fn is_active(&self) -> bool {
        true
    }

    fn deactivate(&mut self) {
        todo!()
    }
}

#[test]
fn effects() {
    let mut queue = UpdateQueue::new();
    let mut messenger1 = Messenger::new("Jessie", "Hello!");
    let mut messenger2 = Messenger::new("tester", "testing");

    queue.push(0, &mut messenger1);
    queue.push(1, &mut messenger2);

    let mut effects: Vec<Effect> = Vec::new();
    let mut log: Vec<Message> = Vec::with_capacity(2);

    while let Some(_) = queue.update(&mut effects) {
        for effect in effects.drain(..) {
            match effect {
                Effect::Log(msg) => log.push(msg)
            }
        }
    }

    assert_eq!(format!("{:?}", log[0]), format!("{:?}", messenger1.msg));
    assert_eq!(format!("{:?}", log[1]), format!("{:?}", messenger2.msg));
}