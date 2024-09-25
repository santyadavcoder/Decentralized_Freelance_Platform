#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Struct to store details of a job contract.
#[contracttype]
#[derive(Clone)]
pub struct JobContract {
    pub job_id: u64,        // Unique ID for the job
    pub freelancer: String, // Freelancer name or address
    pub client: String,     // Client name or address
    pub description: String,// Job description
    pub payment: u64,       // Payment for the job in XLM
    pub is_completed: bool, // Job completion status
}

// For referencing all job contracts.
const ALL_JOBS: Symbol = symbol_short!("ALL_JOBS");

// Struct to track the overall contract status.
#[contracttype]
#[derive(Clone)]
pub struct PlatformStatus {
    pub total_jobs: u64,     // Total number of jobs created on the platform
    pub completed_jobs: u64, // Total number of jobs completed
    pub ongoing_jobs: u64,   // Total number of ongoing jobs
}

// Struct to map job contract to unique IDs.
#[contracttype]
pub enum Jobbook {
    Job(u64),
}

// Counter for job contracts.
const COUNT_JOB: Symbol = symbol_short!("C_JOB");

#[contract]
pub struct FreelancePlatformContract;

#[contractimpl]
impl FreelancePlatformContract {
    
    // Function to create a new job contract.
    pub fn create_job(env: Env, freelancer: String, client: String, description: String, payment: u64) -> u64 {
        let mut count_job: u64 = env.storage().instance().get(&COUNT_JOB).unwrap_or(0);
        count_job += 1;

        let mut all_jobs = Self::view_all_jobs_status(env.clone());

        // Creating new job contract.
        let job_contract = JobContract {
            job_id: count_job,
            freelancer: freelancer.clone(),
            client: client.clone(),
            description: description.clone(),
            payment: payment,
            is_completed: false,
        };

        // Updating platform status.
        all_jobs.total_jobs += 1;
        all_jobs.ongoing_jobs += 1;

        // Storing job contract.
        env.storage().instance().set(&Jobbook::Job(count_job), &job_contract);
        env.storage().instance().set(&ALL_JOBS, &all_jobs);
        env.storage().instance().set(&COUNT_JOB, &count_job);

        log!(&env, "Job created with ID: {}", count_job);

        count_job // Returning unique job ID.
    }

    // Function to complete a job and release payment.
    pub fn complete_job(env: Env, job_id: u64) {
        let mut job_contract = Self::view_job_by_id(env.clone(), job_id);
        let mut all_jobs = Self::view_all_jobs_status(env.clone());

        // Checking if the job is already completed.
        if job_contract.is_completed {
            log!(&env, "Job {} is already completed.", job_id);
            panic!("Job is already completed.");
        }

        // Marking job as completed.
        job_contract.is_completed = true;
        all_jobs.completed_jobs += 1;
        all_jobs.ongoing_jobs -= 1;

        // Storing the updated job contract and platform status.
        env.storage().instance().set(&Jobbook::Job(job_id), &job_contract);
        env.storage().instance().set(&ALL_JOBS, &all_jobs);

        log!(&env, "Job {} is now marked as completed.", job_id);
    }

    // Function to view the status of all jobs on the platform.
    pub fn view_all_jobs_status(env: Env) -> PlatformStatus {
        env.storage().instance().get(&ALL_JOBS).unwrap_or(PlatformStatus {
            total_jobs: 0,
            completed_jobs: 0,
            ongoing_jobs: 0,
        })
    }

    // Function to view details of a specific job by its ID.
    pub fn view_job_by_id(env: Env, job_id: u64) -> JobContract {
        let key = Jobbook::Job(job_id);

        env.storage().instance().get(&key).unwrap_or(JobContract {
            job_id: 0,
            freelancer: String::from_str(&env, "Not_Found"),
            client: String::from_str(&env, "Not_Found"),
            description: String::from_str(&env, "Not_Found"),
            payment: 0,
            is_completed: false,
        })
    }
}
