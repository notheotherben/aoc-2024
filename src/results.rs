use std::io::Write;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

static RESULTS: ResultContainer = ResultContainer::new();

pub fn run<D: ToString, V: ToString, F>(day: D, run: F) -> Result<(), Box<dyn std::error::Error>>
    where F: FnOnce() -> Result<V, Box<dyn std::error::Error>>
{
    RESULTS.run(day, run)
}

struct ResultContainer(Mutex<Vec<PuzzleResult>>, AtomicUsize);

impl ResultContainer {
    const fn new() -> Self {
        Self(Mutex::new(Vec::new()), AtomicUsize::new(0))
    }

    fn run<D: ToString, V: ToString, F>(&self, day: D, run: F) -> Result<(), Box<dyn std::error::Error>>
        where F: FnOnce() -> Result<V, Box<dyn std::error::Error>>
    {
        self.1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let start = std::time::Instant::now();
        let (result, err) = match run() {
            Ok(result) => (result.to_string(), None),
            Err(err) => (format!("{}", err), Some(err)),
        };

        let duration = start.elapsed();

        let mut results = self.0.lock().unwrap();
        results.push(PuzzleResult(day.to_string(), result, duration));
        self.1.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);

        err.map(Err).unwrap_or(Ok(()))
    }

    fn wait_for_complete(&self) {
        while self.1.load(std::sync::atomic::Ordering::Relaxed) > 0 {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

impl Drop for ResultContainer {
    fn drop(&mut self) {
        #[allow(clippy::explicit_write)]
        writeln!(std::io::stdout(), "{}", self).unwrap();
    }
}

impl std::fmt::Display for ResultContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let results = self.0.lock().unwrap();

        let mut results = results.clone();
        results.sort();

        for result in results.into_iter() {
            writeln!(f, "{}", result)?;
        }

        Ok(())
    }

}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PuzzleResult(String, String, Duration);

impl Ord for PuzzleResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.len().cmp(&other.0.len()) {
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            ordering => ordering,
        }
    }
}

impl PartialOrd for PuzzleResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for PuzzleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.2 > Duration::from_millis(2) {
            write!(f, "{}: {} ({}ms)", self.0, self.1, self.2.as_millis())
        } else {
            write!(f, "{}: {} ({}µs)", self.0, self.1, self.2.as_micros())            
        }
    }
}

#[test]
fn print() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::sleep(std::time::Duration::from_millis(10));
    RESULTS.wait_for_complete();

    writeln!(std::io::stdout())?;
    writeln!(std::io::stdout(), "Results:")?;
    writeln!(std::io::stdout(), "{}", RESULTS)?;

    Ok(())
}
  