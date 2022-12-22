use anyhow::Result;
use public_stash::models::PublicStashChange;
use testcontainers::{Container, Docker, Image};

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

const EXAMPLE_STASH_CHANGE: &'static str = include_str!("example-stash.json");

// pub async fn insert_raw_items(repo: &RawItemRepository) -> Result<()> {
//     let mut tr = repo.begin().await?;

//     let stash: PublicStashChange = serde_json::from_str(EXAMPLE_STASH_CHANGE)?;
//     for i in stash.items {
//         repo.insert_item(
//             &mut tr,
//             RawItem::new(
//                 i.id.as_ref().unwrap(),
//                 stash.account_name.as_ref().unwrap(),
//                 stash.stash.as_ref().unwrap(),
//                 i.clone(),
//             ),
//         )
//         .await?;
//     }

//     tr.commit().await?;
//     Ok(())
// }
