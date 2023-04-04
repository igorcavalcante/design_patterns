pub trait Item {
    fn price(&self) -> f64;
    fn add_item(&mut self, _: Box<dyn Item>) -> Result<(), &str>{
        return Err("not implemented on a simple item")
    }
    fn remove_item(&mut self, _: usize) -> Result<(), &str>{
        return Err("not implemented on a simple item")
    }
}

struct ItemComposite {
    items: Vec<Box<dyn Item>>
}

impl ItemComposite {
    fn new() -> Self {
        return ItemComposite{
            items: Vec::new()
        }
    }
}

impl Item for ItemComposite {
    fn price(&self) -> f64 {
        let prices = self.items.iter().map(|x| x.price());
        return prices.sum()
    }

    fn add_item(&mut self, i: Box<dyn Item>) -> Result<(), &str>{
        self.items.push(i);
        return Ok(())
    }

    fn remove_item(&mut self, position: usize) -> Result<(), &str>{
        _ = self.items.remove(position);
        return Ok(())
    }
}

pub struct Lens {
    composite: ItemComposite,
    pub(crate) price: f64,
}

impl Lens {
    pub fn new(price: f64) -> Self {
        return Lens{
            composite : ItemComposite::new(),
            price,
        }
    }
}


impl Item for Lens {
    fn price(&self) -> f64 {
        return &self.price + &self.composite.price()
    }

    fn add_item(&mut self, i: Box<dyn Item>) -> Result<(), &str>{
        return self.composite.add_item(i);
    }

    fn remove_item(&mut self, i: usize) -> Result<(), &str>{
        return self.composite.remove_item(i);
    }
}

pub struct Frame {
    composite: ItemComposite,
    price: f64,
}

impl Item for Frame {
    fn price(&self) -> f64 {
        return &self.price + &self.composite.price()
    }

    fn add_item(&mut self, i: Box<dyn Item>) -> Result<(), &str>{
        return self.composite.add_item(i);
    }

    fn remove_item(&mut self, i: usize) -> Result<(), &str>{
        return self.composite.remove_item(i)
    }
}

impl Frame {
    pub fn new(price: f64) -> Self {
        return Frame{
            composite : ItemComposite::new(),
            price,
        }
    }
}

pub struct Treatment {
    pub price: f64,
}

impl Item for Treatment {
    fn price(&self) -> f64 {
        return self.price
    }
}

impl Treatment {
    pub fn new(price: f64) -> Self {
        return Treatment{
            price,
        }
    }
}

pub struct Signature {
    pub price: f64,
}

impl Signature {
    pub fn new(price: f64) -> Self {
        return Signature{
            price,
        }
    }
}

impl Item for Signature {
    fn price(&self) -> f64 {
        return self.price
    }
}