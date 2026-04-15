#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub is_completed: bool,
}

const TASK_LIST: Symbol = symbol_short!("TASKS");

#[contract]
pub struct TaskFlowContract;

#[contractimpl]
impl TaskFlowContract {
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage().instance().get(&TASK_LIST).unwrap_or(Vec::new(&env))
    }

    pub fn create_task(env: Env, title: String) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_LIST).unwrap_or(Vec::new(&env));
        
        let new_task = Task {
            id: env.prng().gen::<u64>(),
            title: title,
            is_completed: false,
        };
        
        tasks.push_back(new_task);
        env.storage().instance().set(&TASK_LIST, &tasks);
        
        String::from_str(&env, "Task created success")
    }

    pub fn toggle_status(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_LIST).unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();
            if task.id == id {
                task.is_completed = !task.is_completed;
                tasks.set(i, task);
                
                env.storage().instance().set(&TASK_LIST, &tasks);
                return String::from_str(&env, "Task status updated");
            }
        }
        
        String::from_str(&env, "Task not found")
    }

    pub fn remove_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_LIST).unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);
                env.storage().instance().set(&TASK_LIST, &tasks);
                return String::from_str(&env, "Task deleted success");
            }
        }
        
        String::from_str(&env, "Task not found")
    }
}

mod test;
