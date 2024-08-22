use clap::Parser;

#[derive(Parser, Debug)]
pub struct ListCli {
    /// A formatter for projects, `{}` will be replaced with the name, if omitted only the name
    /// will be used
    #[arg(long = "format-project", short = 'p', alias = "fmt-proj")]
    pub format_project: Option<String>,

    /// A formatter for templates, `{}` will be replaced with the name, if omitted only the name
    /// will be used
    #[arg(long = "format-template", short = 't', alias = "fmt-temp")]
    pub format_template: Option<String>,

    /// A formatter for directories, `{}` will be replaced with the name, if omitted only the name
    /// will be used
    #[arg(long = "format-directory", short = 'd', alias = "fmt-dir")]
    pub format_directory: Option<String>,
}
