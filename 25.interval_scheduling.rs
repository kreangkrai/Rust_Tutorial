use chrono::LocalResult;
use chrono::prelude::*;
use chrono::TimeZone;

#[derive(Debug,Clone)]
struct JOB {
    job: &'static str,
    start: LocalResult<DateTime<Local>>,
    stop: LocalResult<DateTime<Local>>,
}
#[derive(Debug,Clone)]
struct JOBS {
    jobs: Vec<JOB>,
}
impl JOBS {
    fn new() -> Self {
        JOBS { jobs: Vec::new() }
    }
    fn insert(&mut self, job: JOB) {
        self.jobs.push(job);
    }
    fn sortbystop(&mut self){
        self.jobs.sort_by_key(|f| f.stop.earliest());
    }
    fn interval_scheduling(&mut self) -> Vec<JOB>{
        self.sortbystop();
        let mut ans = vec![];
        let mut last_stop = self.jobs.first().unwrap().start;

        for j in self.jobs.clone().into_iter(){
                if j.start.unwrap() >= last_stop.unwrap(){
                ans.push(j.clone());
                last_stop = j.stop;
            }
        }
        ans
    }
    fn print_interval_scheduling(&mut self){       
        println!("JOB               START                              STOP");
        for a in self.interval_scheduling().iter(){
            println!("{:?}      {:?}        {:?}",a.job,a.start.unwrap(),a.stop.unwrap());
        }
    }
}
fn main() {
    let mut jobs = JOBS::new();
    jobs.insert(JOB {
        job: "A",
        start: Local.with_ymd_and_hms(2022, 12, 10, 9, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 10, 0, 0),
    });
    jobs.insert(JOB {
        job: "B",
        start: Local.with_ymd_and_hms(2022, 12, 10, 9, 30, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 10, 30, 0),
    });
    jobs.insert(JOB {
        job: "C",
        start: Local.with_ymd_and_hms(2022, 12, 10, 10, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 11, 0, 0),
    });
    jobs.insert(JOB {
        job: "D",
        start: Local.with_ymd_and_hms(2022, 12, 10, 11, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 12, 30, 0),
    });
    jobs.insert(JOB {
        job: "E",
        start: Local.with_ymd_and_hms(2022, 12, 10, 10, 30, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 13, 0, 0),
    });
    jobs.insert(JOB {
        job: "F",
        start: Local.with_ymd_and_hms(2022, 12, 10, 12, 30, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 14, 0, 0),
    });
    jobs.insert(JOB {
        job: "G",
        start: Local.with_ymd_and_hms(2022, 12, 10, 13, 30, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 15, 0, 0),
    });
    jobs.insert(JOB {
        job: "H",
        start: Local.with_ymd_and_hms(2022, 12, 10, 14, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 15, 30, 0),
    });
    jobs.insert(JOB {
        job: "I",
        start: Local.with_ymd_and_hms(2022, 12, 10, 15, 30, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 17, 0, 0),
    });
    jobs.insert(JOB {
        job: "J",
        start: Local.with_ymd_and_hms(2022, 12, 10, 15, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 16, 30, 0),
    });
    jobs.print_interval_scheduling();
}
