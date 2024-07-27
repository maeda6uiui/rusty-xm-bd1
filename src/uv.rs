use std::fmt;

#[derive(Debug,Clone)]
pub struct UV{
    pub u: f32,
    pub v: f32,
}

impl fmt::Display for UV{
    fn fmt(&self,f: &mut fmt::Formatter)->fmt::Result{
        write!(f,"({},{})",self.u,self.v)
    }
}
