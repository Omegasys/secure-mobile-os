use std::path::PathBuf;

#[derive(Clone)]
pub enum TransferStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
}

#[derive(Clone)]
pub struct TransferJob {
    pub id: u64,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub bytes_total: u64,
    pub bytes_transferred: u64,
    pub status: TransferStatus,
}

pub struct TransferManager {
    jobs: Vec<TransferJob>,
}

impl TransferManager {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
        }
    }

    pub fn add_job(
        &mut self,
        job: TransferJob,
    ) {
        self.jobs.push(job);
    }

    pub fn update_progress(
        &mut self,
        id: u64,
        transferred: u64,
    ) {
        if let Some(job) =
            self.jobs.iter_mut().find(|j| j.id == id)
        {
            job.bytes_transferred = transferred;

            if transferred >= job.bytes_total {
                job.status =
                    TransferStatus::Completed;
            }
        }
    }

    pub fn jobs(&self) -> &[TransferJob] {
        &self.jobs
    }
}
