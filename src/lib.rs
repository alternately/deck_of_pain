extern crate rand;

pub mod intensity {

    use std::io;
    use rand::Rng;

    fn set_intensity() -> i8 {
        

        let mut intensity: i8; // variable declared outside the loop to be in the right scope

        //if the user inputs an invalid answer we let them try again (sometimes)
        loop{
            
            // get user input
            println!("On a scale from 1-10, how intense do you want your workout to be");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("problem reading user input");

            // turn the user input into Result<i8>
            let intensity_result = user_input.trim().parse::<i8>();
            // unwrap the user input with some error handling
            match intensity_result {
                Ok(i) => {
                    intensity = i;
                    break;
                },
                Err(e) => {
                    println!("you fucked up, and got the error: {}", e);
                    println!("Try Again!");
                }
            }

        }

        // this block checks that intensity is between one and ten, and if it isn't it sets it to 10 by way of punishment
        if intensity > 0 && intensity < 11 {
            return intensity
        } else {
            println!("you couldn't even enter a number between 1 and 10, so we figured we'd just set your workout to 10. Good luck.");
            intensity = 10;
            return intensity
        }

            
    }

    fn get_random_variation() -> i8 {

        // all this function does is generate a random number between -2 and 2
        let mut rng = super::rand::thread_rng();
        let variation: i8 = rng.gen_range(-2, 3);
        variation
            
    }

    pub fn intensity() -> i8 {

        let mut final_intensity: i8 = set_intensity() + get_random_variation();
        // this block handles negative sums, which are possible in rare cases

        if final_intensity < 1 {
            final_intensity = 1;
        }
        
        final_intensity
    }

    

}

pub mod exercises {

    use rand::Rng;

    pub struct Exercise {
        pub name: String,
        pub reps_per_spot: f32,
    }

    pub enum ExerciseTypes {
        PushUp,
        PullUp,
        Mile,
    }

    impl ExerciseTypes {

        pub fn value(&self) -> Exercise {

            match &self {
                &&ExerciseTypes::PushUp => Exercise {name: String::from("push-up"), reps_per_spot: 5.0},
                &&ExerciseTypes::PullUp => Exercise {name: String::from("pull-up"), reps_per_spot: 1.0},
                &&ExerciseTypes::Mile => Exercise {name: String::from("mile"), reps_per_spot: 0.5},
            }
        }

        pub fn reporting_phrase(&self, spots: f32) -> String {

            let reps = spots * &self.value().reps_per_spot;

            match &self {
                &&ExerciseTypes::PushUp => String::from(format!("do {} push-ups!", reps)),
                &&ExerciseTypes::PullUp => String::from(format!("do {} pull-ups!", reps)),
                &&ExerciseTypes::Mile => String::from(format!("run {} miles", reps)),
            }
        }
    }

    pub fn generate_exercises() -> Vec<(ExerciseTypes, f32)> {

        let mut rng = super::rand::thread_rng();
        let mut intensity = super::intensity::intensity();
        let mut v1: Vec<(ExerciseTypes, f32)> = Vec::new();

        while intensity > 0 {

            let spots = rng.gen_range(1, 14);
            let spots = spots as f32;
            if &spots > &8.0 {
                intensity = intensity - 3;
            } else if &spots > &4.0 {
                intensity = intensity - 2;
            } else {
                intensity = intensity - 1;
            }

            let ex_type = rng.gen_range(0, 3);

            if ex_type == 0 {
                v1.push((ExerciseTypes::PushUp, spots));
            } else if ex_type == 1 {
                v1.push((ExerciseTypes::PullUp, spots));
            } else if ex_type == 2 {
                v1.push((ExerciseTypes::Mile, spots));
            } else {
                panic!("random generation of exercise types failed");
            }
        }
            
        
        v1
    }

    pub fn generate_report(invec: Vec<(ExerciseTypes, f32)>) {
        for tup in invec {
            let (exercise, spots) = tup;
            println!("{}", exercise.reporting_phrase(spots));
        }
    }
        
        

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn intensity_range() {        
        assert!(intensity::set_intensity() < 11 && intensity::set_intensity() > 0);
    }
}
