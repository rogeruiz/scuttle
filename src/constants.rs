pub mod structurizr {

    #[derive(Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
    pub enum Formatters {
        PlantUML,
        PlantUMLStructurizr,
        PlantUMLC4,
        Mermaid,
        WebSequence,
        Ilograph,
        DOT,
        JSON,
        DSL,
        Theme,
    }
}
