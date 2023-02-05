pub mod structurizr {

    #[derive(Copy, Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
    pub enum Formatters {
        PlantUML,
        Mermaid,
    }
}
