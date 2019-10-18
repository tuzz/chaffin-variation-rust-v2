use build_const::ConstWriter;
use lehmer::Lehmer;

fn main() {








    let mut consts = ConstWriter::for_build("constants")
        .unwrap().finish_dependencies();

    consts.add_value("MAX", "usize", Lehmer::max_value(6));
}
