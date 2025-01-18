use dotenv::dotenv;
use std::env;
use unifi_rs::{UnifiClient, UnifiClientBuilder, UnifiError};
use uuid::Uuid;

async fn create_test_client() -> UnifiClient {
    dotenv().ok();

    let base_url = env::var("UNIFI_BASE_URL").expect("UNIFI_BASE_URL must be set");
    let api_key = env::var("UNIFI_API_KEY").expect("UNIFI_API_KEY must be set");

    UnifiClientBuilder::new(base_url)
        .api_key(api_key)
        .verify_ssl(false)
        .build()
        .expect("Failed to create client")
}

async fn get_test_site_id(client: &UnifiClient) -> Uuid {
    let sites = client
        .list_sites(None, None)
        .await
        .expect("Failed to list sites");

    sites.data.first().expect("No sites available").id
}

#[tokio::test]
async fn test_list_sites() {
    let client = create_test_client().await;

    let sites = client
        .list_sites(None, None)
        .await
        .expect("Failed to list sites");

    println!("{:?}", sites);

    assert!(!sites.data.is_empty(), "No sites returned");
    let site = &sites.data[0];
    if let Some(name) = &site.name {
        assert!(!name.is_empty(), "Site name is empty");
    }
}

#[tokio::test]
async fn test_list_devices() {
    let client = create_test_client().await;
    let site_id = get_test_site_id(&client).await;

    let devices = client
        .list_devices(site_id, None, None)
        .await
        .expect("Failed to list devices");

    println!("{:?}", devices);
    println!("Found {} devices", devices.data.len());
}

#[tokio::test]
async fn test_device_details() {
    let client = create_test_client().await;
    let site_id = get_test_site_id(&client).await;

    let devices = client
        .list_devices(site_id, None, None)
        .await
        .expect("Failed to list devices");

    println!("{:?}", devices);

    if let Some(device) = devices.data.first() {
        let details = client
            .get_device_details(site_id, device.id)
            .await
            .expect("Failed to get device details");

        println!("{:?}", details);

        assert_eq!(details.id, device.id);
        assert_eq!(details.name, device.name);
        assert!(!details.mac_address.is_empty());
    } else {
        println!("No devices available to test details");
    }
}

#[tokio::test]
async fn test_device_statistics() {
    let client = create_test_client().await;
    let site_id = get_test_site_id(&client).await;

    let devices = client
        .list_devices(site_id, None, None)
        .await
        .expect("Failed to list devices");

    println!("{:?}", devices);

    if let Some(device) = devices.data.first() {
        let stats = client
            .get_device_statistics(site_id, device.id)
            .await
            .expect("Failed to get device statistics");

        println!("{:?}", stats);

        assert!(stats.uptime_sec >= 0);
        if let Some(cpu) = stats.cpu_utilization_pct {
            assert!((0.0..=100.0).contains(&cpu));
        }
        if let Some(mem) = stats.memory_utilization_pct {
            assert!((0.0..=100.0).contains(&mem));
        }
    } else {
        println!("No devices available to test statistics");
    }
}

#[tokio::test]
async fn test_list_clients() {
    let client = create_test_client().await;
    let site_id = get_test_site_id(&client).await;

    let clients = client
        .list_clients(site_id, None, None)
        .await
        .expect("Failed to list clients");

    println!("{:?}", clients);

    println!("Found {} clients", clients.data.len());

    if let Some(client_overview) = clients.data.first() {
        match client_overview {
            unifi_rs::ClientOverview::Wired(c) => {
                assert!(!c.mac_address.is_empty());
            }
            unifi_rs::ClientOverview::Wireless(c) => {
                assert!(!c.mac_address.is_empty());
            }
            _ => {}
        }
    }
}

#[tokio::test]
async fn test_get_info() {
    let client = create_test_client().await;

    let info = client
        .get_info()
        .await
        .expect("Failed to get application info");

    assert!(!info.application_version.is_empty());
}

#[tokio::test]
async fn test_pagination() {
    let client = create_test_client().await;
    let site_id = get_test_site_id(&client).await;

    let page1 = client
        .list_devices(site_id, Some(0), Some(1))
        .await
        .expect("Failed to get first page");

    assert!(page1.limit == 1);

    if page1.total_count > 1 {
        let page2 = client
            .list_devices(site_id, Some(1), Some(1))
            .await
            .expect("Failed to get second page");

        if let (Some(dev1), Some(dev2)) = (page1.data.first(), page2.data.first()) {
            assert_ne!(dev1.id, dev2.id, "Pagination returned same device");
        }
    }
}

#[tokio::test]
async fn test_error_handling() {
    let client = UnifiClientBuilder::new("https://example.com")
        .api_key("invalid-key")
        .verify_ssl(false)
        .build()
        .expect("Failed to create client");

    let result = client.list_sites(None, None).await;

    match result {
        Err(UnifiError::Api { status_code, .. }) => {
            assert!(status_code >= 400, "Expected error status code");
        }
        Err(UnifiError::Http(e)) => {
            println!("Got HTTP error as expected: {}", e);
        }
        _ => panic!("Expected error response"),
    }
}
