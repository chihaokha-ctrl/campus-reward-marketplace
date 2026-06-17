#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String,
};

#[derive(Clone)]
#[contracttype]
pub struct Student {
    pub name: String,
    pub points: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct Reward {
    pub name: String,
    pub cost: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Student(Address),
    Reward(String),
}

#[contract]
pub struct CampusRewardMarketplace;

#[contractimpl]
impl CampusRewardMarketplace {
    // =========================
    // ADMIN SETUP
    // =========================

    pub fn initialize(env: Env, admin: Address) {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Admin, &admin);
    }

    fn get_admin(env: &Env) -> Address {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .unwrap()
    }

    // =========================
    // STUDENT MANAGEMENT
    // =========================

    pub fn register_student(
        env: Env,
        student: Address,
        name: String,
    ) {
        student.require_auth();

        let student_data = Student {
            name,
            points: 0,
        };

        env.storage()
            .persistent()
            .set(
                &DataKey::Student(student),
                &student_data,
            );
    }

    pub fn get_student(
        env: Env,
        student: Address,
    ) -> Student {
        env.storage()
            .persistent()
            .get(&DataKey::Student(student))
            .unwrap()
    }

    pub fn get_points(
        env: Env,
        student: Address,
    ) -> u32 {
        let data: Student = env
            .storage()
            .persistent()
            .get(&DataKey::Student(student))
            .unwrap();

        data.points
    }

    // =========================
    // POINT MANAGEMENT
    // =========================

    pub fn add_points(
        env: Env,
        admin: Address,
        student: Address,
        amount: u32,
    ) {
        admin.require_auth();

        let stored_admin = Self::get_admin(&env);

        if admin != stored_admin {
            panic!("not admin");
        }

        let mut data: Student = env
            .storage()
            .persistent()
            .get(&DataKey::Student(student.clone()))
            .unwrap();

        data.points += amount;

        env.storage()
            .persistent()
            .set(
                &DataKey::Student(student),
                &data,
            );
    }

    // =========================
    // REWARD MANAGEMENT
    // =========================

    pub fn create_reward(
        env: Env,
        admin: Address,
        reward_id: String,
        reward_name: String,
        cost: u32,
    ) {
        admin.require_auth();

        let stored_admin = Self::get_admin(&env);

        if admin != stored_admin {
            panic!("not admin");
        }

        let reward = Reward {
            name: reward_name,
            cost,
        };

        env.storage()
            .persistent()
            .set(
                &DataKey::Reward(reward_id),
                &reward,
            );
    }

    pub fn get_reward(
        env: Env,
        reward_id: String,
    ) -> Reward {
        env.storage()
            .persistent()
            .get(&DataKey::Reward(reward_id))
            .unwrap()
    }

    // =========================
    // REDEMPTION
    // =========================

    pub fn redeem_reward(
        env: Env,
        student: Address,
        reward_id: String,
    ) -> bool {
        student.require_auth();

        let reward: Reward = env
            .storage()
            .persistent()
            .get(&DataKey::Reward(reward_id))
            .unwrap();

        let mut data: Student = env
            .storage()
            .persistent()
            .get(&DataKey::Student(student.clone()))
            .unwrap();

        if data.points < reward.cost {
            return false;
        }

        data.points -= reward.cost;

        env.storage()
            .persistent()
            .set(
                &DataKey::Student(student),
                &data,
            );

        true
    }
}