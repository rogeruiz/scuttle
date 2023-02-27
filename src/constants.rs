//! The constants that are used to validate, verify, and ensure that Scuttle remains compatible
//! with Structurizr-CLI and the various outputs it can generate, such as PlantUML.

pub mod structurizr {
    //! The structurizr module is here to create helpful functions, enums, etc when working with the
    //! Structurizr-CLI.

    /// Formatters are valid formats that Structurizr-CLI supports. Not all formats are available
    /// from Scuttle at this time. Read more about supported formats for Structurizr-CLI here,
    /// <https://github.com/structurizr/cli/blob/master/docs/export.md#options>
    #[derive(Copy, Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
    pub enum Formatters {
        /// The PlantUML format. Read more about it here, <https://plantuml.com/>
        PlantUML,
        /// The Mermaid format. Read more about it here, <https://mermaid.js.org/>
        Mermaid,
    }
}
