use badge_maker::{BadgeBuilder, Style};
use clap::{Parser, ValueEnum};

#[derive(clap::Parser, Debug)]
#[clap(name = "badge-maker")]
struct Opts {
    label: String,
    message: String,
    #[clap(short, long, default_value("lightgrey"))]
    color: String,
    #[clap(short, long, default_value("grey"))]
    label_color: String,
    #[clap(short, long, default_value("flat"))]
    style: ClapStyle,
}
#[derive(Debug, Clone, Copy)]
pub struct ClapStyle(Style);
impl ValueEnum for ClapStyle {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ClapStyle(Style::Flat),
            ClapStyle(Style::Plastic),
            ClapStyle(Style::FlatSquare),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self.0 {
            Style::Flat => Some(clap::builder::PossibleValue::new("flat")),
            Style::Plastic => Some(clap::builder::PossibleValue::new("plastic")),
            Style::FlatSquare => Some(clap::builder::PossibleValue::new("flatsquare")),
        }
    }
    fn from_str(input: &str, _: bool) -> Result<Self, String> {
        input.parse().map(ClapStyle).map_err(|e| e.to_string())
    }
}
fn main() {
    let opts: Opts = Opts::parse();
    match BadgeBuilder::new()
        .message(&opts.message)
        .label(&opts.label)
        .color_parse(&opts.color)
        .label_color_parse(&opts.label_color)
        .style(opts.style.0)
        .build()
    {
        Ok(badge) => println!("{}", badge.svg()),
        Err(e) => {
            println!("Badge Error {:?}", e);
        }
    }
}
