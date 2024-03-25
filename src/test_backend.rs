use reqwest::Error;

use crate::softwares;

pub struct TestBackend;

impl TestBackend {
    pub async fn send_post(url: &str, body: &str) {
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .body(body.to_string())
            .send()
            .await
            .unwrap_or_else(|err| {
                eprintln!("Failed to send POST request: {}", err);
                panic!("Failed to send POST request");
            });
        if response.status().is_success() {
            println!("POST request was successful!");
            let s = response.text().await.unwrap_or_else(|err| {
                eprintln!("Failed to read response body: {}", err);
                panic!("Failed to read response body");
            });
            println!("{}", s);
        } else {
            println!("POST request failed with status: {}", response.status());
        }
    }

    pub async fn send_get(url: &str) ->Result<String, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .unwrap_or_else(|err| {
                eprintln!("Failed to send GET request: {}", err);
                panic!("Failed to send GET request");
            });
        let s = response.text().await.unwrap_or_else(|err| {
            eprintln!("Failed to read response body: {}", err);
            panic!("Failed to read response body");
        });
        println!("发送完毕");
        Ok(s)
    }

    pub async fn test_get_file_url_by_archive_version(archive: &str, version: &str)-> Result<String, Error> {
        let url = format!("http://127.0.0.1:8080/api/v1/softwares/information?version={}&archive={}", version, archive);
        //let url = format!("http://127.0.0.1:8080/api/v1/softwares/file?archive={}&version={}", archive, version);
        let r = Self::send_get(&url).await;
        r
    }
    pub async fn test_add() -> Result<String, Error>{
        let archive = "test".to_string();
        let version = "1.0.1".to_string();
        let version_major = 1;
        let version_minor = 0;
        let version_patch = 0;
        let component = "test".to_string();
        let origin = "test".to_string();
        let label = "test".to_string(); 
        let architecture = "test".to_string();
        let download = "test".to_string();
        let others = "test".to_string();
        let flag = 1;
        //let model = softwares::Model::new(archive, version, version_major, version_minor, version_patch, component, origin, label, architecture, download, others, flag);
        let url = format!("http://127.0.0.1:8080/api/v1/softwares/add_software?archive={}&version={}&version_major={}&version_minor={}&version_patch={}&component={}&origin={}&label={}&architecture={}&download={}&others={}&flag={}",
            archive, version, version_major, version_minor, version_patch, component, origin, label, architecture, download, others, flag);
        let r = Self::send_get(&url).await;
        r
    }
    pub async fn test_delete() -> Result<String, Error>{
        let version = "0.0.0".to_string();
        let archive = "test".to_string();
        let url = format!("http://127.0.0.1:8080/api/v1/softwares/delete_software?version={}&archive={}", version, archive);
        let r = Self::send_get(&url).await;
        r
    }
    pub async fn test_update() -> Result<String, Error>{
        let archive = "test".to_string();
        let version = "1.0.1".to_string();
        let version_major = 1;
        let version_minor = 0;
        let version_patch = 0;
        let component = "test".to_string();
        let origin = "test".to_string();
        let label = "20240316".to_string(); 
        let architecture = "test".to_string();
        let download = "test".to_string();
        let others = "aerith".to_string();
        let flag = 1;
        //let model = softwares::Model::new(archive, version, version_major, version_minor, version_patch, component, origin, label, architecture, download, others, flag);
        let url = format!("http://127.0.0.1:8080/api/v1/softwares/update_software?archive={}&version={}&version_major={}&version_minor={}&version_patch={}&component={}&origin={}&label={}&architecture={}&download={}&others={}&flag={}",
            archive, version, version_major, version_minor, version_patch, component, origin, label, architecture, download, others, flag);
        let r = Self::send_get(&url).await;
        r
    }
}
