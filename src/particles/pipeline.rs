use crate::particles::operations::operation::Operation;


pub struct Pipeline(Vec<Box<dyn Operation>>);

impl Pipeline {
    pub fn push(&mut self, value: Box<dyn Operation>) {
        self.0.push(value);
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            0: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::particles::operations::merge::Merge;

    use super::*;

    #[test]
    fn default() {
        let mut p = Pipeline::default();
        assert_eq!(p.0.len(), 0);

        p.push(Box::new(Merge::default()));
    }
}