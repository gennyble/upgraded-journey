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

    pub fn run_struct<T: 'static>(&self, s: &dyn Runnable<T>) {
        let type_id = TypeId::of::<T>();
        match self.data.get(&type_id) {
            Some(vec) => {
                for data in vec.iter() {
                    if let Some(cast) = data.downcast_ref::<T>() {
                        s.run(cast);
                    }
                }
            }
            None => panic!("We have no data of that type!")
        }
    }
}

trait Runnable<T> {
    fn run(&self, value: &T);
}

struct Manager<'s> {
    workers: HashMap<TypeId, Vec<&'s dyn Any>>
}

impl<'s> Manager<'s> {
    fn new() -> Self {
        Self {workers: HashMap::new()}
    }

    fn add_worker<G: Any + 'static, T: Any + Runnable<G>>(&mut self, s: &'s T) {
        let runtype = TypeId::of::<G>();
        println!("{:?}", runtype);

        match self.workers.get_mut(&runtype) {
            Some(vec) => {
                vec.push(s);
            },
            None => {
                self.workers.insert(runtype, vec![s]);
            }
        }
    }

    fn work_on<T: 'static>(&self, job: &T) {
        let jobtype = TypeId::of::<T>();
        println!("{:?}", jobtype);

        match self.workers.get(&jobtype) {
            Some(vec) => {
                for worker in vec {
                    if let Some(cast) = worker.downcast_ref::<&'s dyn Runnable<T>>() {
                        cast.run(job);
                    }
                }
            },
            None => panic!("No workers to compelte this task!")
        }
    }
}

struct Bar {
    member: String
}

impl Bar {
    fn new() -> Self {
        Self { member: "Bar".into() }
    }
}

impl Runnable<&str> for Bar {
    fn run(&self, string: &&str) {
        println!("{} - {}", self.member, string);
    }
}

impl Runnable<Thing> for Bar {
    fn run(&self, thing: &Thing) {
        println!("Hey, I found a thing! It's name is {} and it's a size of {}! My member is '{}'!", thing.name, thing.size, self.member);
    }
}

struct Tee {
    member: String
}

impl Tee {
    fn new() -> Self {
        Self { member: "Tee".into() }
    }
}

impl Runnable<(f32, f32)> for Tee {
    fn run(&self, other: &(f32, f32)) {
        println!("I'm {} and the other, summed, is {}", self.member, other.0 + other.1);
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
    storage.add((23f32, 73f32));
    storage.add((62f32, 18f32));

    storage.run_fn(&thing_doer);
    storage.run_fn(&str_doer);
    storage.run_fn(&tuplef32f32_doer);

    do_run_struct(&storage);
    do_workers();
}

fn do_run_struct(storage: &Foo) {
    // If Runnable is implemented more than once, type annotations are needed
    // because the compiler doesn't know which one to call
    let bar = Bar::new();
    storage.run_struct::<&str>(&bar);
    storage.run_struct::<Thing>(&bar);

    // Awesomely, if there is only once implementation, the compiler knows
    // the type!
    let tee = Tee::new();
    storage.run_struct(&tee);
}

fn do_workers() {
    println!("Workers should be running...");

    let bar = Bar::new();
    let tee = Tee::new();
    let mut manager = Manager::new();

    manager.add_worker::<&str, _>(&bar);
    manager.add_worker::<Thing, _>(&bar);
    manager.add_worker(&tee);

    manager.work_on(&"Hello");
    manager.work_on(&Thing{
        name: "B-TH-12".into(),
        size: 2
    });
}  

fn thing_doer(thing: &Thing) {
    println!("Hey, I found a thing! It's name is {} and it's a size of {}!", thing.name, thing.size);
}

fn str_doer(s: &&str) {
    println!("I found a &str! It says '{}'", s);
}

fn tuplef32f32_doer(tuple: &(f32, f32)) {
    println!("I found a tuple! ({}, {})", tuple.0, tuple.1);
}