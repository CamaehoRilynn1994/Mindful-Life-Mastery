/* LIFE MASTERY TRAINING & COACHING PROGRAM IN RUST */

// Define program structs
struct TrainingProgram {
    title: String,
    duration: u32,
    modules: Vec<Module>
}

struct Module {
    name: String,
    objectives: Vec<Objective>,
    section_names: Vec<String>
}

struct Objective {
    name: String,
    description: String
}

// Define program data
let training_program = TrainingProgram{
    title: "Life Mastery Training and Coaching".into(),
    duration: 36,
    modules: vec![
        Module {
            name: "Create a Vision for Your Life".into(),
            objectives: vec![
                Objective{
                    name: "Understand Your Values".into(),
                    description: "Learn how to identify your core values and use them to create a purposeful vision.".into()
                },
                Objective{
                    name: "Connect With Your Intuition".into(),
                    description: "Learn how to tap into your intuition and gain insight on what direction to go in life.".into()
                },
                Objective{
                    name: "Set Goals".into(),
                    description: "Learn how to set achievable and meaningful goals that align with your values and vision.".into()
                },
            ],
            section_names: vec!["Values", "Intuition", "Goals"],
        },
        Module {
            name: "Develop Self-Awareness".into(),
            objectives: vec![
                Objective{
                    name: "Develop Emotional Intelligence".into(),
                    description: "Learn how to become more self-aware and better understand your emotions and how they impact your life.".into()
                },
                Objective{
                    name: "Understand Self-Talk".into(),
                    description: "Learn how to identify and change negative self-talk, and replace it with more positive and supportive dialogue.".into()
                },
                Objective{
                    name: "Discover Your Strengths".into(),
                    description: "Learn how to identify and leverage your personal strengths to achieve success in life.".into()
                },
            ],
            section_names: vec!["Emotional Intelligence", "Self-Talk", "Strengths"],
        },
        Module {
            name: "Increase Self-Confidence".into(),
            objectives: vec![
                Objective{
                    name: "Understand Self-Worth".into(),
                    description: "Learn how to develop a strong sense of self-worth, and use it to become more confident in life.".into()
                },
                Objective{
                    name: "Develop Effective Habits".into(),
                    description: "Learn how to develop and maintain effective habits that contribute to your long-term success.".into()
                },
                Objective{
                    name: "Enhance Self-Respect".into(),
                    description: "Learn how to cultivate self-respect by embracing your personal journey and recognizing your individual potential.".into()
                },
            ],
            section_names: vec!["Self-Worth", "Habits", "Self-Respect"],
        },
    ]
};

// Define program functions
fn print_module_info(module: &Module) {
    println!("Module: {}", module.name);
    println!("Objectives:");
    for (i, objective) in module.objectives.iter().enumerate() {
        println!("  {}. {}: {}", i+1, objective.name, objective.description);
    }
    println!("Section Names:");
    for (i, section_name) in module.section_names.iter().enumerate() {
        println!("  {}. {}", i+1, section_name);
    }
    println!("");
}

fn print_program_info(program: &TrainingProgram) {
    println!("Training Program: {}", program.title);
    println!("Duration: {} Weeks", program.duration);
    println!("Modules:\n");
    for module in program.modules.iter() {
        print_module_info(module);
    }
}

// Execute program
fn main() {
    print_program_info(&training_program);
}