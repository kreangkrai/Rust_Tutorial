use chrono::prelude::*;
use chrono::LocalResult;
use chrono::TimeZone;

#[derive(Debug,Clone)]
struct Queue<T> {
    queue: Vec<T>,
}
#[allow(dead_code)]
impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }
    fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }
    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
    fn length(&self) -> usize {
        self.queue.len()
    }
    fn isempty(&self) -> bool {
        self.queue.is_empty()
    }
    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
    fn last(&self) -> Option<&T>{
        self.queue.last()
    }
}
#[derive(Debug, Clone, Copy)]
struct JOB {
    job: &'static str,
    start: LocalResult<DateTime<Local>>,
    stop: LocalResult<DateTime<Local>>,
}
#[derive(Debug, Clone)]
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
    fn sortbystart(&mut self) {
        self.jobs.sort_by_key(|f| f.start.earliest());
    }
    fn interval_partitioning(&mut self) -> Vec<Queue<JOB>> {
        self.sortbystart();
        let mut jobs_que: Queue<JOB> = Queue::new();
        let mut ans: Vec<Queue<JOB>> = Vec::new();
        for j in self.jobs.clone().into_iter() {
            jobs_que.enqueue(j);
        }
        let mut j1 = jobs_que.peek().unwrap();

        ans.push(Queue { queue: Vec::new() });
        ans[0].enqueue(j1.to_owned());
        jobs_que.dequeue();

        let mut d = 1;

        while !jobs_que.isempty() {
            j1 = jobs_que.peek().unwrap();
         
            let mut j2 = ans.iter().map(|m| m.last().unwrap()).into_iter().collect::<Vec<&JOB>>();
            let temp_j2 = j2.clone();
            let b = j2.iter().any(|a| a.stop.unwrap() <= j1.start.unwrap());
            if b {
                j2 = j2
                    .clone()
                    .into_iter()
                    .filter(|f| f.stop.unwrap() <= j1.start.unwrap()).collect::<Vec<&JOB>>();

                    j2.sort_by_key(|&f| f.start.unwrap());

                let j3 = j2.clone().into_iter().last().unwrap();
                let index = temp_j2
                    .into_iter()
                    .position(|p| p.job == j3.job)
                    .unwrap();
                ans[index].enqueue(j1.to_owned());
            } else {
                ans.push(Queue::new());
                ans[d].enqueue(j1.to_owned());
                d += 1;
            }
            jobs_que.dequeue();
        }
        ans
    }
    fn print_interval_partitioning(&mut self){
        for i in 0..self.interval_partitioning().len() {
            println!("=========================== {} =============================",i + 1);
            println!("JOB            START                         STOP");
            for q in self.interval_partitioning()[i].queue.iter() {
                println!("{:?}   {:?}   {:?}",q.job,q.start.unwrap(),q.stop.unwrap());
            }
        }
    }
}
fn main() {
    let mut jobs = JOBS::new();
    jobs.insert(JOB {
        job: "A",
        start: Local.with_ymd_and_hms(2022, 12, 10, 9, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 10, 30, 0),
    });
    jobs.insert(JOB {
        job: "B",
        start: Local.with_ymd_and_hms(2022, 12, 10, 9, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 12, 30, 0),
    });
    jobs.insert(JOB {
        job: "C",
        start: Local.with_ymd_and_hms(2022, 12, 10, 9, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 10, 30, 0),
    });
    jobs.insert(JOB {
        job: "D",
        start: Local.with_ymd_and_hms(2022, 12, 10, 11, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 12, 30, 0),
    });
    jobs.insert(JOB {
        job: "E",
        start: Local.with_ymd_and_hms(2022, 12, 10, 11, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 14, 0, 0),
    });
    jobs.insert(JOB {
        job: "F",
        start: Local.with_ymd_and_hms(2022, 12, 10, 13, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 14, 30, 0),
    });
    jobs.insert(JOB {
        job: "G",
        start: Local.with_ymd_and_hms(2022, 12, 10, 13, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 14, 30, 0),
    });
    jobs.insert(JOB {
        job: "H",
        start: Local.with_ymd_and_hms(2022, 12, 10, 14, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 16, 30, 0),
    });
    jobs.insert(JOB {
        job: "I",
        start: Local.with_ymd_and_hms(2022, 12, 10, 15, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 16, 30, 0),
    });
    jobs.insert(JOB {
        job: "J",
        start: Local.with_ymd_and_hms(2022, 12, 10, 15, 0, 0),
        stop: Local.with_ymd_and_hms(2022, 12, 10, 16, 30, 0),
    });
    jobs.print_interval_partitioning();
}
