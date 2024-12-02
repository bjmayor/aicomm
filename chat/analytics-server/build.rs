use std::{fs, process::Command};

use anyhow::Result;
use proto_builder_trait::prost::BuilderAttributes;
fn main() -> Result<()> {
    // 编译到了 OUT_DIR 目录下
    // prost_build::compile_protos(&["../protos/crm.proto"], &["../protos"])?;
    // 自定义输出目录
    fs::create_dir_all("./src/pb/")?;
    // let mut config = prost_build::Config::new();
    // config
    //     .out_dir("./src/pb/")
    //     .compile_protos(&["../protos/crm.proto"], &["../protos"])?;
    prost_build::Config::new()
        .out_dir("./src/pb/")
        // .with_derive_builder(&["WelcomeRequest", "RecallRequest", "RemindRequest"], None)
        // .with_field_attributes(
        //     &["WelcomeRequest.content_ids"],
        //     &[r#"#[builder(setter(each(name="content_id", into)))]"#],
        // )
        .compile_protos(&["../../protos/messages.proto"], &["../../protos"])?;
    Command::new("cargo").args(["fmt"]).output().unwrap();
    Ok(())
}
