pub mod command {

    use terminal_spinners::{SpinnerBuilder, DOTS2};

    use bollard::container::{
        Config, CreateContainerOptions, LogOutput, LogsOptions, RemoveContainerOptions,
        StartContainerOptions,
    };
    use bollard::image::CreateImageOptions;
    use bollard::models::{HostConfig, Mount};
    use bollard::Docker;

    use futures_util::stream::StreamExt;
    use futures_util::stream::TryStreamExt;

    const IMAGE: &str = "structurizr/cli:latest";

    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    #[tokio::main]
    pub async fn run_export(
        format: &str,
        workspace: &str,
        output: &str,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        // Connecting to Docker via default socket
        let docker = Docker::connect_with_socket_defaults().unwrap();

        let sd1 = docker.clone();

        // Create the Structurizr-CLI Docker configuration
        let structurizr_config = Config {
            image: Some(IMAGE),
            // The host_config property was the only way I was able to ensure I could mount the
            // local current working directory to where the Structurizr-CLI image expects the
            // working directory to be.
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
                "export", "-f", &format, "-w", &workspace, "-o", &output,
            ]),
            tty: Some(false),
            ..Default::default()
        };

        let running_export = SpinnerBuilder::new()
            .spinner(&DOTS2)
            .text(" Starting the structurizr/cli (via Docker)")
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
                stderr: true,
                ..Default::default()
            }),
        );

        // Since the stream is
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(output) => {
                    let mut output_stream = StandardStream::stdout(ColorChoice::Always);
                    match output {
                        LogOutput::StdOut { .. } => {
                            output_stream.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                        }
                        LogOutput::StdErr { .. } => {
                            output_stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                        }
                        _ => (),
                    }
                    writeln!(&mut output_stream, "{}", output).ok();
                }
                Err(_) => unreachable!(),
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
