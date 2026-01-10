//! HEOS Telnet Client
//!
//! Handles the low-level telnet connection to HEOS devices.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::types::{HeosConfig, HeosError, HeosHeader, HeosResponse};

/// HEOS telnet client for sending commands and receiving responses
pub struct HeosClient {
    config: HeosConfig,
    stream: Arc<Mutex<Option<TcpStream>>>,
}

impl HeosClient {
    /// Create a new HEOS client with the given configuration
    pub fn new(config: HeosConfig) -> Self {
        Self {
            config,
            stream: Arc::new(Mutex::new(None)),
        }
    }

    /// Create a new HEOS client with default configuration
    pub fn with_defaults() -> Self {
        Self::new(HeosConfig::default())
    }

    /// Update the configuration (e.g., change host or player_id)
    pub fn set_config(&mut self, config: HeosConfig) {
        self.config = config;
        // Disconnect to force reconnect with new config
        self.disconnect();
    }

    /// Get the current player ID
    pub fn player_id(&self) -> i64 {
        self.config.player_id
    }

    /// Set the player ID
    pub fn set_player_id(&mut self, pid: i64) {
        self.config.player_id = pid;
    }

    /// Connect to the HEOS device
    pub fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let stream = TcpStream::connect_timeout(&addr.parse()?, Duration::from_secs(5))?;

        // Set read timeout
        stream.set_read_timeout(Some(Duration::from_secs(10)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;

        let mut guard = self.stream.lock().unwrap();
        *guard = Some(stream);

        Ok(())
    }

    /// Get the current host
    pub fn host(&self) -> &str {
        &self.config.host
    }

    /// Disconnect from the HEOS device
    pub fn disconnect(&self) {
        let mut guard = self.stream.lock().unwrap();
        *guard = None;
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        let guard = self.stream.lock().unwrap();
        guard.is_some()
    }

    /// Ensure connection is established, reconnect if needed
    fn ensure_connected(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_connected() {
            self.connect()?;
        }
        Ok(())
    }

    /// Send a raw HEOS command and return the raw JSON response
    pub fn send_command(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.ensure_connected()?;

        let mut guard = self.stream.lock().unwrap();
        let stream = guard.as_mut().ok_or("Not connected")?;

        // Send command with CRLF terminator
        let full_command = format!("{}\r\n", command);
        stream.write_all(full_command.as_bytes())?;
        stream.flush()?;

        // Read response until CRLF
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut response = String::new();
        reader.read_line(&mut response)?;

        Ok(response.trim().to_string())
    }

    /// Send a command and parse the response as JSON
    pub fn send_command_parsed<T>(
        &self,
        command: &str,
    ) -> Result<HeosResponse<T>, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let response = self.send_command(command)?;
        let parsed: HeosResponse<T> = serde_json::from_str(&response)?;

        // Check for errors in response
        if !parsed.heos.is_success() {
            let msg = parsed.heos.parse_message();
            let code = msg.get("eid").and_then(|s| s.parse().ok()).unwrap_or(-1);
            let text = msg
                .get("text")
                .cloned()
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(Box::new(HeosError {
                code,
                message: text,
            }));
        }

        Ok(parsed)
    }

    /// Send a command and only check for success (no payload expected)
    pub fn send_command_simple(
        &self,
        command: &str,
    ) -> Result<HeosHeader, Box<dyn std::error::Error>> {
        let response = self.send_command(command)?;
        let parsed: HeosResponse<()> = serde_json::from_str(&response)?;

        if !parsed.heos.is_success() {
            let msg = parsed.heos.parse_message();
            let code = msg.get("eid").and_then(|s| s.parse().ok()).unwrap_or(-1);
            let text = msg
                .get("text")
                .cloned()
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(Box::new(HeosError {
                code,
                message: text,
            }));
        }

        Ok(parsed.heos)
    }
}

impl Clone for HeosClient {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            stream: Arc::new(Mutex::new(None)), // New instance gets its own connection
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_parse_message() {
        let header = HeosHeader {
            command: "player/get_volume".to_string(),
            result: "success".to_string(),
            message: "pid=123&level=50".to_string(),
        };

        let parsed = header.parse_message();
        assert_eq!(parsed.get("pid"), Some(&"123".to_string()));
        assert_eq!(parsed.get("level"), Some(&"50".to_string()));
    }
}
