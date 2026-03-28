pub mod state_machine {
    use std::marker::PhantomData;

    // zero size types for encoding state
    pub struct Red;
    pub struct Green;
    pub struct Yellow;

    pub struct StateMachine<T> {
        pub name: &'static str,
        pub _state: PhantomData<T>
    }

    impl StateMachine<Red> {
        pub fn go_green(&self) -> StateMachine<Green> {
            StateMachine { name: "Green", _state: PhantomData } 
        } 
    } 

    impl StateMachine<Green> {
        pub fn go_yellow(&self) -> StateMachine<Yellow> {
            StateMachine { name: "Yellow", _state: PhantomData } 
        }
    }

    impl StateMachine<Yellow> {
        pub fn go_red(&self) -> StateMachine<Red> {
            StateMachine { name: "Red", _state: PhantomData } 
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::state_machine::*;
    use std::marker::PhantomData;

    #[test]
    pub fn basics() {
        let sm_r = StateMachine::<Red> { name: "Red", _state: PhantomData };

        let sm_g = sm_r.go_green();
        assert_eq!(sm_g.name, "Green"); // no need to add these assertions, because if program
                                        // compiled then the type must be correct; we can also
                                        // declare the type of binding variable explicitly to make
                                        // this more clear
 
        let sm_y = sm_g.go_yellow();

        let sm_r = sm_y.go_red();
    }
}


