// mod constants;
// use crate::constants::structurizr;

pub mod command {

    use bollard::container::{
        Config, CreateContainerOptions, LogsOptions, RemoveContainerOptions, StartContainerOptions,
    };
    use bollard::image::CreateImageOptions;
    use bollard::Docker;

    use futures_util::stream::StreamExt;
    use futures_util::stream::TryStreamExt;

    const IMAGE: &str = "structurizr/cli:latest";

    #[tokio::main]
    pub async fn run_export() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let docker = Docker::connect_with_socket_defaults().unwrap();

        let sd1 = docker.clone();

        let structurizr_config = Config {
            image: Some(IMAGE),
            cmd: Some(vec!["structurizr.sh"]),
            ..Default::default()
        };

        let _ = &docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: IMAGE,
                    ..Default::default()
                }),
                None,
                None,
            )
            .try_collect::<Vec<_>>()
            .await?;

        let id = &docker
            .create_container(
                Some(CreateContainerOptions {
                    name: "structurizr",
                }),
                structurizr_config,
            )
            .await?
            .id;

        let _ = &docker
            .start_container("structurizr", None::<StartContainerOptions<String>>)
            .await?;

        let mut stream = sd1.logs::<String>(
            "structurizr",
            Some(LogsOptions {
                follow: true,
                stdout: true,
                stderr: false,
                ..Default::default()
            }),
        );

        while let Some(msg) = stream.next().await {
            println!("Message: {:#?}", msg);
        }

        docker
            .remove_container(
                &id,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;

        Ok(())
    }
}
