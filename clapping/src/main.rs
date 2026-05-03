use clap::Parser;

#[derive(Parser)]
#[command(name = "clapping")]
struct Cli {
    #[arg(short, long)]
    algorithm: String,
    #[arg(short = 'n', long, default_value = "10")]
    size: usize,
}

const BANNER: &str = r#"
    /\_____/\
   /  o   o  \
  ( ==  ^  == )
   )         (
  (           )
 ( (  )   (  ) )
(__(__)___(__)__)

  ┌─────────────────────────────┐
  │   algoviz v0.1.0            │
  │   algorithm visualizer      │
  └─────────────────────────────┘
"#;

fn main() {
    let cli = Cli::parse();

    println!("{}", BANNER);
    println!("algorithm: {}", cli.algorithm);
    println!("size : {}", cli.size);
    println!();
    println!("starting!");
}
