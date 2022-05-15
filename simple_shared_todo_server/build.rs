

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute("ItemInfo", r#"#[derive(serde::Serialize, serde::Deserialize)]"#)
        .compile(&["../proto/service/service.proto"], &["../proto/service"])?;

    Ok(())
}