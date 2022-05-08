use std::env;
use testcontainers::{
    clients::Cli,
    images::generic::{GenericImage, Stream, WaitFor},
    images::postgres::Postgres,
    Container, Docker, Image, RunArgs,
};
use tokio::process::*;
use tokio::select;

pub struct ContainerDrop<'d, D: Docker, I: Image> {
    pub container: Container<'d, D, I>,
}

impl<'d, D: Docker, I: Image> Drop for ContainerDrop<'d, D, I> {
    fn drop(&mut self) {
        self.container.stop();

        let mut container_stdout = String::new();
        self.container
            .logs()
            .stdout
            .read_to_string(&mut container_stdout)
            .unwrap();
        let mut container_stderr = String::new();
        self.container
            .logs()
            .stderr
            .read_to_string(&mut container_stderr)
            .unwrap();

        println!("container stdout: {}", container_stdout);
        println!("container stderr: {}", container_stderr);

        self.container.rm();
    }
}

