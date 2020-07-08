use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Debug))]
struct MyStruct {
    #[structopt(long)]
    value: String,
}

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Debug))]
struct AnotherStruct {
    #[structopt(flatten)]
    #[configopt(nowrap)]
    my_struct: ConfigOptMyStruct,
}

#[test]
fn test_no_wrap() {}
