use eyre::Result;

pub fn create_task(task: &str) -> Result<()> {
    println!("Running tool 'create task' with task '{task}'");

    Ok(())
}
