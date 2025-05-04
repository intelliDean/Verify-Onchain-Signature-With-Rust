use qrcode::QrCode;
use qrcode::render::svg;
use serde_json::json;

fn generate_qr_code(certificate: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    // Convert certificate to JSON string
    let cert_str = serde_json::to_string(certificate)?;

    // Generate QR code
    let code = QrCode::new(cert_str.as_bytes())?;

    // Render as SVG (for web frontend)
    let svg = code.render::<svg::Color>()
        .min_dimensions(200, 200)
        .build();

    Ok(svg)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example certificate
    let certificate = json!({
        "serial": "XM12345",
        "imei": "543210987654321",
        "model": "Redmi Note 14",
        "date": "2025-05-01",
        "manufacturerAddress": "0x5678901234567890123456789012345678901234",
        "signature": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
    });

    let svg = generate_qr_code(&certificate)?;
    println!("{}", svg); // Output SVG for frontend
    Ok(())
}