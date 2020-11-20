use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;

struct Foo {
    data: HashMap<TypeId, Vec<Box<dyn Any>>>
}

impl Foo {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn add<T: Any>(&mut self, data: T) {
        let type_id = TypeId::of::<T>();
        let boxed = Box::new(data);

        match self.data.get_mut(&type_id) {
            Some(vec) => {
                vec.push(boxed);
            },
            None => {
                self.data.insert(type_id, vec![boxed]);
            }
        }
    }

    pub fn run_fn<T: 'static>(&self, f: &dyn Fn(&T)) {
        let type_id = TypeId::of::<T>();
        match self.data.get(&type_id) {
            Some(vec) => {
                for data in vec.iter() {
                    if let Some(cast) = data.downcast_ref::<T>() {
                        f(cast);
                    }
                }
            }
            None => ()
        }
    }
}

struct Thing {
    pub name: String,
    pub size: u8
}

fn main() {
    let mut storage = Foo::new();
    storage.add("Dog");
    storage.add("Cat");
    storage.add(Thing {
        name: "AX-12-AB".into(),
        size: 2
    });
    storage.add(Thing {
        name: "AX-13-AB".into(),
        size: 7
    });

    storage.run_fn(&thing_doer);
    storage.run_fn(&str_doer);
}

fn thing_doer(thing: &Thing) {
    println!("Hey, I found a thing! It's name is {} and it's a size of {}!", thing.name, thing.size);
}

fn str_doer(s: &&str) {
    println!("I found a &str! It says '{}'", s);
}
