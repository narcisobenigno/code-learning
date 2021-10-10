use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

type ProviderRef = Rc<RefCell<_Provider>>;

struct _Provider {
    connected: Vec<ProviderRef>,
    does: String,
    name: String,
}

struct Provider(ProviderRef);

impl Provider {
    pub fn new(name: &str, does: &str) -> Self {
        Self(Rc::new(RefCell::new(_Provider{
            connected: vec![],
            does: does.to_string(),
            name: name.to_string(),
        })))
    }

    pub fn connect(&self, provider: &Provider) {
        (self.0.borrow_mut()).connected.push(provider.0.clone());
    }

    pub fn connected(&self) -> Vec<Provider> {
        (self.0.borrow_mut()).connected.iter()
            .map(|x| Provider(x.clone()))
            .collect()
    }

    pub fn do_you_do(&self, what: &str) -> bool {
        (self.0.borrow_mut()).does == what
    }

    pub fn name(&self) -> String {
        (self.0.borrow_mut()).name.clone()
    }
}

fn breadth_first(providers: VecDeque<Provider>, does: &str) -> Option<String> {
    if providers.is_empty() {
        return None
    }

    let mut next_level: VecDeque<Provider> = VecDeque::new();

    for c in providers {
        if c.do_you_do(does) {
            return Some(c.name())
        }
        for p in c.connected() {
            next_level.push_back(p);
        }
    }

    breadth_first(next_level, does)
}

pub fn provider<'a>(connections: Vec<(&'a str, &'a str)>, duties: Vec<(&'a str, &'a str)>, whom: &'a str, does: &'a str) -> Option<String> {
    let providers: &mut HashMap<&'a str, Provider> = &mut HashMap::new();
    for (name, dutie) in duties {
        providers.insert(
            name,
            Provider::new(name, dutie),
        );
    }

    for (p1, p2) in connections {
        providers[p2].connect(&providers[p1]);
    }

    let provider = &providers[whom];

    let mut queue: VecDeque<Provider> = VecDeque::new();

    for p in provider.connected() {
        queue.push_back(p);
    }

    breadth_first(queue, does)
}

#[cfg(test)]
mod test {
    use crate::algorithms::provider;

    #[test]
    fn it_finds_service_in_network() {
        let connections = vec![
            ("Hegel", "Marx"),
            ("Engels", "Marx"),
            ("Proudhon", "Marx"),
            ("Engels", "Kautsky"),
            ("Kautsky", "Ulyanov"),
            ("Ulyanov", "Lukács")
        ];

        let known = vec![
            ("Hegel", "Idealism"),
            ("Engels", "Materialist"),
            ("Proudhon", "Reformist"),
            ("Marx", "Materialist"),
            ("Kautsky", "Reformist"),
            ("Ulyanov", "Revolutionary"),
            ("Lukács", "Arts"),
        ];

        assert_eq!(
            Some("Proudhon".to_string()),
            provider(connections, known, "Marx", "Reformist")
        )
    }
}