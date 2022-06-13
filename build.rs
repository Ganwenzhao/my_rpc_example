fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["proto/my_proto.proto"], &["."])
        .expect("failed to compile protos");
}