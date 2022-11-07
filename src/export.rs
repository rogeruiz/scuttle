// mod constants;
// use crate::constants::structurizr;

pub mod command {

    use bollard::container::{
        Config, CreateContainerOptions, LogsOptions, RemoveContainerOptions, StartContainerOptions,
    };
    use bollard::image::CreateImageOptions;
    use bollard::Docker;

    use zzz::prelude::*;

    use futures_util::stream::StreamExt;
    use futures_util::stream::TryStreamExt;

    const IMAGE: &str = "structurizr/cli:latest";

    #[tokio::main]
    pub async fn run_export() -> Result<(), Box<dyn std::error::Error + 'static>> {
        // Connecting to Docker via default socket
        let docker = Docker::connect_with_socket_defaults().unwrap();

        let sd1 = docker.clone();

        // Create the Structurizr-CLI Docker configuration
        let structurizr_config = Config {
            image: Some(IMAGE),
            cmd: Some(vec!["structurizr.sh"]),
            ..Default::default()
        };

        // Create the Image and show a progress bar
        let _ = &docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: IMAGE,
                    ..Default::default()
                }),
                None,
                None,
            )
            .progress()
            .try_collect::<Vec<_>>()
            .await?;

        // Create the container
        let id = &docker
            .create_container(
                Some(CreateContainerOptions {
                    name: "structurizr",
                }),
                structurizr_config,
            )
            .await?
            .id;

        // Start the container
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

        // Since the stream is
        while let Some(msg) = stream.next().await {
            println!("Message: {:#?}", msg);
        }

        // Let's remove the container since we're done here.
        docker
            .remove_container(
                &id,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;

        // TODO: Learn what this is doing here so I can better handle error handling when something
        // doesn't work the way I think it will.
        Ok(())
    }
}
