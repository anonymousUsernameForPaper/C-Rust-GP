use std::rc::Rc;
use crate::components::cgp_components::chromosome_evaluator_operators::{ChromosomeEvaluation, ChromosomeEvaluatorGeneral};
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::components::evo_operators_for_population::evaluation_operators::eval_population_trait::PopulationGeneralForwardPass;
use crate::function_set::function_trait::Function;
use crate::utils::runner::ProgramState;

pub struct PopulationForwardPassOnePlusFour;


impl<T: Clone> PopulationGeneralForwardPass<T> for PopulationForwardPassOnePlusFour
where ChromosomeEvaluatorGeneral: ChromosomeEvaluation<T> {
    fn new() -> Box<dyn PopulationGeneralForwardPass<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self, runner: &mut ProgramState<T>,
               evaluator_function: Rc<Box<dyn ChromosomeEvaluation<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>) {
        // because of (1+4), there should only be one elitist
        assert_eq!(1, runner.elitist_ids.len());
        assert_eq!(4, runner.child_ids.len());

        let parent_id = runner.elitist_ids[0];
        for id in 0..(runner.params.elitists + runner.params.population_size) {
            if id != parent_id {
                let fitness: f32 = evaluator_function.evaluate(&mut runner.population[id],
                                                                   Rc::clone(&active_node_func),
                                                                   &runner.data,
                                                                   &runner.label,
                                                                   Rc::clone(&function_set));
                runner.fitness_vals[id] = fitness;
            }
        }
        runner.sort_fitness_vals();
    }


    fn get_test_fitness(&self, runner: &mut ProgramState<T>, evaluator_function: Rc<Box<dyn ChromosomeEvaluation<T>>>, active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>, function_set: Rc<Vec<Box<dyn Function<T>>>>) -> f32 {
        // because of (1+4), there should only be one elitist
        assert_eq!(1, runner.elitist_ids.len());
        assert_eq!(4, runner.child_ids.len());

        let mut best_fitness = f32::MAX;
        assert!(runner.eval_data.is_some());

        for id in 0..(runner.params.elitists + runner.params.population_size) {
            let fitness: f32 = evaluator_function.evaluate(&mut runner.population[id],
                                                           Rc::clone(&active_node_func),
                                                           runner.eval_data.as_ref().unwrap(),
                                                           runner.eval_label.as_ref().unwrap(),
                                                           Rc::clone(&function_set));
            if fitness < best_fitness {
                best_fitness = fitness;
            }
        }

        best_fitness
    }
}


