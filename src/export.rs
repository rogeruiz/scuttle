pub mod command {

    use terminal_spinners::{SpinnerBuilder, DOTS8_BIT};

    use bollard::container::{
        Config, CreateContainerOptions, LogsOptions, RemoveContainerOptions, StartContainerOptions,
    };
    use bollard::image::CreateImageOptions;
    use bollard::models::{HostConfig, Mount};
    use bollard::Docker;

    use futures_util::stream::StreamExt;
    use futures_util::stream::TryStreamExt;

    const IMAGE: &str = "structurizr/cli:latest";

    use std::io::Write;
    use std::path::PathBuf;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    #[tokio::main]
    pub async fn run_export(
        format: &str,
        workspace: &str,
        output: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        // Connecting to Docker via default socket
        let docker = Docker::connect_with_socket_defaults().unwrap();

        let sd1 = docker.clone();

        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        // Create the Structurizr-CLI Docker configuration
        let structurizr_config = Config {
            image: Some(IMAGE),
            host_config: Some(HostConfig {
                mounts: Some(vec![Mount {
                    target: Some("/usr/local/structurizr".to_string()),
                    source: Some(
                        std::env::current_dir()
                            .unwrap()
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                    ),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                }]),
                // binds: Some(vec!["$PWD:/usr/local/structurizr".to_string()]),
                ..Default::default()
            }),
            cmd: Some(vec![
                "export",
                "-f",
                &format,
                "-w",
                &workspace,
                "-o",
                &output.to_str().unwrap(),
            ]),
            ..Default::default()
        };

        let running_export = SpinnerBuilder::new()
            .spinner(&DOTS8_BIT)
            .text(" Running the structurizr/cli in Docker")
            .start();

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

        running_export.done();

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
            match msg {
                Ok(m) => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                    writeln!(&mut stdout, "{}", m).ok();
                }
                Err(m) => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                    writeln!(&mut stdout, "{}", m).ok();
                }
            }
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
