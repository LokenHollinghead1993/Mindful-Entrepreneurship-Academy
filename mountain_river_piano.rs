// Setting up the module for the Entrepreneurship Training and Mentorship Program 

mod Entrepreneurship_Training_and_Mentorship_Program { 

    // creating an enum for the different types of mentors
    enum MentorTypes {
        Professional,
        Industry,
        Academic,
        Personal,
    }
    
    // creating a struct to store information about the program
    struct Program {
        name: String,
        mentors: Vec<MentorTypes>
    }
    
    // creating a function to create and add a new program
    fn create_and_add_program(name: &str, mentors: Vec<MentorTypes>) -> Program {
        Program {
            name: name.to_owned(),
            mentors
        }
    }
    
    // creating a function to add new mentors to a program
    fn add_mentors(program: &mut Program, mentors: Vec<MentorTypes>) {
        program.mentors.extend(mentors);
    }
    
    // creating a function to remove mentors from a program
    fn remove_mentors(program: &mut Program, mentors: Vec<MentorTypes>) {
        let mut i = 0;
        while i < program.mentors.len() {
            if program.mentors[i] == mentors {
                program.mentors.remove(i);
                i -= 1;
            }
            i += 1;
        }
    }
    
    // creating a function to get the list of mentors from a program
    fn get_mentors(program: &Program) -> Vec<MentorTypes> {
        program.mentors.clone()
    }
    
    // creating a function to print the details of a program
    fn print_program_details(program: &Program) {
        println!("Program name: {}", program.name);
        println!("Mentors:");
        for mentor in &program.mentors {
            println!("- {}", mentor);
        }
    }
    
    // creating function to start the program with a list of mentors
    fn start_program(name: &str, mentors: Vec<MentorTypes>) -> Program {
        let mut program = create_and_add_program(name, mentors);
        print_program_details(&program);
        program
    }
    
    // creating a function to end the program
    fn end_program(program: &Program) {
        println!("Ending program: {}", program.name);
    }
}