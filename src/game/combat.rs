use std::marker::PhantomData;

pub trait AttributeType {}

pub struct Health;
impl AttributeType for Health {}

pub struct Strength;
impl AttributeType for Strength {}

pub struct Magic;
impl AttributeType for Magic {}

pub struct Attribute<T: AttributeType> {
    _marker: PhantomData<T>,
    value: i32,
    listeners: Vec<Box<dyn Fn(i32) -> ()>>,
}

impl<T: AttributeType> Attribute<T> {
    pub fn init(value: i32) -> Self {
        Attribute {
            _marker: PhantomData::<T>,
            value, 
            listeners: vec![]
        }
    }

    pub fn get(&self) -> i32 {
        self.value
    }
    
    pub fn set(&mut self, value: i32) {
        if self.value == value { 
            return; 
        }
        if value < 0 {
            self.value = 0
        } else {
            self.value = value;
        }
        for listener in self.listeners.iter() {
            listener(self.value);
        }
    }

    pub fn add_event_listener(&mut self, listener: Box<dyn Fn(i32) -> ()>){
        self.listeners.push(listener);
    }
}

pub struct Modifier<T: AttributeType> {
    _marker: PhantomData<T>,
    value: i32,
}

impl<T: AttributeType> Modifier<T> {
    pub fn init(value: i32) -> Self {
        Modifier {
            _marker: PhantomData::<T>,
            value 
        }
    }
}

pub struct Entity<'a> {
    name: &'a str,
    pub health: Attribute<Health>,
    pub strength: Attribute<Strength>,
    pub magic: Attribute<Magic>,
}

impl<'a> Entity<'a> {
    pub fn new(name: &'a str) -> Self {
        Entity {
            name,
            health: Attribute::init(0),
            strength: Attribute::init(0),
            magic: Attribute::init(0),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn combat<T: AttributeType>(&self, target: &mut Entity, modifier: Modifier<T>) {
        print!("{} deal {} damage to {}", self.name, modifier.value, target.name);
        target.health.set(target.health.value - modifier.value);
    }
}
