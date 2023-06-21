#[subxt::subxt(runtime_metadata_path = "../resources/goro.metadata")]
pub mod goro {}

use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // API
    let extrinsic_client = OnlineClient::<PolkadotConfig>::from_url("wss://main-00.goro.network:443").await?;

    // Get event from latest block
    let events = extrinsic_client.events().at_latest().await?;

    // Dynamic events
    println!("\nDynamic event details:");
    for event in events.iter() {
        let event = event?;

        let pallet = event.pallet_name();
        let variant = event.variant_name();
        let field_values = event.field_values()?;
        println!("{pallet}::{variant}: {field_values}");
    }

    // Static events
    println!("\nStatic event details:");
    for event in events.iter() {
        let event = event?;
        
        if let Ok(ev) = event.as_root_event::<goro::Event>() {
            println!("{ev:?}");
        } else {
            println!("<Cannot decode event>");
        }
    }

    // Balance transfer
    let transfer_event = events.find_first::<goro::balances::events::Transfer>()?;
    if let Some(ev) = transfer_event {
        println!("\n  - Balance transfer success: value: {:?}", ev.amount);
    } else {
        println!("\n  - No balance transfer event found in this block");
    }

    Ok(())
}
