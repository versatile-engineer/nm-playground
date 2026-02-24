use std::collections::HashMap;
use zbus::{proxy, Connection};

#[proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    fn get_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}

#[proxy(
    interface = "org.freedesktop.NetworkManager.Device",
    default_service = "org.freedesktop.NetworkManager"
)]
trait Device {
    #[zbus(property)]
    fn device_type(&self) -> zbus::Result<u32>;
}

#[proxy(
    interface = "org.freedesktop.NetworkManager.Device.Wireless",
    default_service = "org.freedesktop.NetworkManager"
)]

trait Wireless {
    fn request_scan(&self, options: HashMap<String, zbus::zvariant::Value<'_>>)
        -> zbus::Result<()>;
    fn get_access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}

#[proxy(
    interface = "org.freedesktop.NetworkManager.AccessPoint",
    default_service = "org.freedesktop.NetworkManager"
)]

trait AccessPoint {
    #[zbus(property)]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;

    #[zbus(property)]
    fn strength(&self) -> zbus::Result<u8>;

    #[zbus(property)]
    fn frequency(&self) -> zbus::Result<u32>;
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::system().await?;

    let nm = NetworkManagerProxy::new(&connection).await?;

    let devices = nm.get_devices().await?;

    for device_path in devices {
        let device = DeviceProxy::builder(&connection)
            .path(device_path.clone())?
            .build()
            .await?;

        if device.device_type().await? == 2 {
            println!("WiFi device topildi: {}", device_path);

            let wifi = WirelessProxy::builder(&connection)
                .path(device_path.clone())?
                .build()
                .await?;

            wifi.request_scan(HashMap::new()).await?;

            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            let aps = wifi.get_access_points().await?;

            println!("Mavjud tarmoqlar:");
            for ap_path in aps {
                let ap = AccessPointProxy::builder(&connection)
                    .path(ap_path)?
                    .build()
                    .await?;

                let ssid_bytes = ap.ssid().await?;
                let ssid = String::from_utf8_lossy(&ssid_bytes);
                let strength = ap.strength().await?;

                println!("  SSID: {}, Signal: {}%", ssid, strength);
            }

            break;
        }
    }

    Ok(())
}
