use std::env;

fn read_args(args : env::Args) -> Result<Vec<String>,()>{
    args.collect()
}