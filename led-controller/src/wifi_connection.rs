use esp_idf_svc::wifi::AccessPointInfo;

// TODO_SD: May reuse css part from frontend

const PART_1: &str = r##"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>WiFi Connection</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
    <style>
        :root {
            --primary: #4a6cf7;
            --dark: #1d2a3a;
            --darker: #131a25;
            --light: #f5f8ff;
            --card-bg: #243247;
            --success: #00c896;
            --error: #ff4757;
            --border-radius: 12px;
            --transition: all 0.3s ease;
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, var(--darker) 0%, var(--dark) 100%);
            color: var(--light);
            min-height: 100vh;
            padding: 20px;
            line-height: 1.6;
        }

        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }

        header {
            text-align: center;
            margin-bottom: 40px;
            padding: 20px 0;
        }

        h1 {
            font-size: 2.5rem;
            margin-bottom: 10px;
            background: linear-gradient(90deg, #4a6cf7, #8a63d2);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        .subtitle {
            color: #a0b3d9;
            font-size: 1.1rem;
        }

        .wifi-card {
            background: var(--card-bg);
            border-radius: var(--border-radius);
            padding: 20px;
            box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
            transition: var(--transition);
            display: flex;
            flex-direction: column;
            max-width: 500px;
            margin: 0 auto;
        }

        .wifi-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 15px 30px rgba(0, 0, 0, 0.3);
        }

        .wifi-name {
            font-size: 1.3rem;
            font-weight: 600;
            margin-bottom: 15px;
            color: #fff;
            display: flex;
            align-items: center;
            gap: 10px;
        }

        .wifi-name i {
            color: var(--primary);
        }

        .wifi-list {
            margin-bottom: 20px;
        }

        .wifi-item {
            background: rgba(255, 255, 255, 0.05);
            border-radius: 8px;
            padding: 12px 15px;
            margin-bottom: 10px;
            cursor: pointer;
            transition: var(--transition);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .wifi-item:hover {
            background: rgba(255, 255, 255, 0.1);
        }

        .wifi-item.selected {
            background: rgba(74, 108, 247, 0.2);
            border: 1px solid var(--primary);
        }

        .wifi-item .signal-strength {
            display: flex;
            gap: 2px;
        }

        .wifi-item .signal-bar {
            width: 4px;
            height: 10px;
            background: #a0b3d9;
            border-radius: 2px;
        }

        .wifi-item .signal-bar.active {
            background: var(--success);
        }

        .password-section {
            margin-top: 20px;
            display: none;
        }

        .password-section.active {
            display: block;
        }

        .password-label {
            display: block;
            margin-bottom: 8px;
            font-size: 0.9rem;
            color: #a0b3d9;
        }

        .password-input {
            width: 100%;
            height: 42px;
            border: none;
            border-radius: 8px;
            padding: 0 15px;
            background: rgba(255, 255, 255, 0.1);
            color: var(--light);
            font-size: 1rem;
            margin-bottom: 15px;
        }

        .password-input:focus {
            outline: 2px solid var(--primary);
        }

        .connect-btn {
            background: var(--primary);
            color: white;
            border: none;
            border-radius: 8px;
            padding: 12px 20px;
            font-weight: 600;
            cursor: pointer;
            transition: var(--transition);
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            width: 100%;
        }

        .connect-btn:hover {
            background: #5b7aff;
            transform: translateY(-2px);
        }

        .status-message {
            margin-top: 15px;
            padding: 10px;
            border-radius: 8px;
            text-align: center;
            display: none;
        }

        .status-message.error {
            background: rgba(255, 71, 87, 0.2);
            color: var(--error);
            display: block;
        }

        .status-message.success {
            background: rgba(0, 200, 150, 0.2);
            color: var(--success);
            display: block;
        }

        .status-message.connecting {
            background: rgba(74, 108, 247, 0.2);
            color: var(--primary);
            display: block;
        }

        .status-indicator {
            display: inline-block;
            width: 10px;
            height: 10px;
            border-radius: 50%;
            margin-left: 10px;
        }

        .status-connected {
            background-color: var(--success);
        }

        footer {
            text-align: center;
            margin-top: 50px;
            padding: 20px;
            color: #a0b3d9;
            font-size: 0.9rem;
        }

        @media (max-width: 768px) {
            h1 {
                font-size: 2rem;
            }
        }
    </style>
</head>

<body>
    <div class="container">
        <header>
            <h1><i class="fas fa-wifi"></i> WiFi Connection</h1>
            <p class="subtitle">Connect to a wireless network</p>
        </header>

        <div class="wifi-card">
            <div class="wifi-name">
                <i class="fas fa-network-wired"></i> Available Networks
                <span class="status-indicator status-connected"></span>
            </div>

            <div class="wifi-list">
"##;

const PART_2: &str = r##"
            </div>

            <div class="password-section" id="passwordSection">
                <span class="password-label">Enter Password for <span id="selectedSsid"></span></span>
                <input type="password" class="password-input" id="passwordInput" placeholder="Password">
                <button class="connect-btn" id="connectBtn">
                    <i class="fas fa-plug"></i> Connect
                </button>
            </div>

            <div class="status-message" id="statusMessage"></div>
        </div>
    </div>

    <footer>
        <p>WiFi Connection Panel © 2025</p>
    </footer>

    <script>
        // Get DOM elements
        const wifiItems = document.querySelectorAll('.wifi-item');
        const passwordSection = document.getElementById('passwordSection');
        const selectedSsid = document.getElementById('selectedSsid');
        const passwordInput = document.getElementById('passwordInput');
        const connectBtn = document.getElementById('connectBtn');
        const statusMessage = document.getElementById('statusMessage');

        // Track selected WiFi
        let selectedWifi = null;

        // Add click event listeners to WiFi items
        wifiItems.forEach(item => {
            item.addEventListener('click', () => {
                // Remove selected class from all items
                wifiItems.forEach(i => i.classList.remove('selected'));
                
                // Add selected class to clicked item
                item.classList.add('selected');
                
                // Store selected WiFi data
                selectedWifi = {
                    ssid: item.dataset.ssid,
                    protected: item.dataset.protected === 'true'
                };
                
                // Update UI based on whether WiFi is protected
                if (selectedWifi.protected) {
                    selectedSsid.textContent = selectedWifi.ssid;
                    passwordSection.classList.add('active');
                    passwordInput.value = '';
                    passwordInput.focus();
                    statusMessage.className = 'status-message';
                } else {
                    passwordSection.classList.remove('active');
                    // Immediately connect to unprotected WiFi
                    connectToWifi(selectedWifi.ssid);
                }
            });
        });

        // Add click event listener to connect button
        connectBtn.addEventListener('click', () => {
            if (!selectedWifi) {
                showStatus('Please select a WiFi network first', 'error');
                return;
            }
            
            if (selectedWifi.protected) {
                const password = passwordInput.value.trim();
                if (!password) {
                    showStatus('Please enter a password', 'error');
                    return;
                }
                connectToWifi(selectedWifi.ssid, password);
            } else {
                connectToWifi(selectedWifi.ssid);
            }
        });

        // Allow pressing Enter to connect
        passwordInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                connectBtn.click();
            }
        });

        // Function to connect to WiFi
        function connectToWifi(ssid, password = null) {
            // Prepare request data
            const requestData = { ssid };
            if (password) {
                requestData.password = password;
            }
            
            // Show connecting status with blue color
            showStatus('Connecting to ' + ssid + '...', 'connecting');
            
            // Send POST request
            fetch('/connect_to_wifi', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(requestData)
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then(data => {
                // Handle successful connection
                showStatus('Successfully connected to ' + ssid, 'success');
                // Reset selection after successful connection
                setTimeout(() => {
                    wifiItems.forEach(i => i.classList.remove('selected'));
                    passwordSection.classList.remove('active');
                    selectedWifi = null;
                    statusMessage.className = 'status-message';
                }, 3000);
            })
            .catch(error => {
                // Handle connection failure
                console.error('Error connecting to WiFi:', error);
                showStatus('Failed to connect to ' + ssid, 'error');
            });
        }

        // Function to show status message
        function showStatus(message, type) {
            statusMessage.textContent = message;
            statusMessage.className = 'status-message ' + type;
        }
    </script>
</body>

</html>
"##;

const NUMBER_OF_BARS: i8 = 4;
const LOWEST_ACCEPTABLE_CONNECTION_QUALITY: i8 = -100;
const ACTIVE_BAR: &str = r##"
<div class="signal-bar active"></div>
"##;
const INACTIVE_BAR: &str = r##"
<div class="signal-bar"></div>
"##;

fn signal_strength(signal_strength: i8) -> String {
    let inactive_bars = signal_strength / (LOWEST_ACCEPTABLE_CONNECTION_QUALITY / NUMBER_OF_BARS); // -100 -> 4, -75 -> 3, -50 -> 2
    let active_bars = NUMBER_OF_BARS - inactive_bars;

    let mut output = "".to_owned();

    for _ in 0..active_bars {
        output = format!("{output}{ACTIVE_BAR}")
    }
    for _ in active_bars..NUMBER_OF_BARS {
        output = format!("{output}{INACTIVE_BAR}")
    }

    output
}

fn wifi_item(ap_info: &AccessPointInfo) -> String {
    let signal_strength = signal_strength(ap_info.signal_strength);
    let ssid = ap_info.ssid.clone();
    let is_protected = !ap_info.auth_method.is_none();

    format!(
        r##"
    <div class="wifi-item" data-ssid="{ssid}" data-protected="{is_protected}">
        <span>{ssid}</span>
        <div class="signal-strength">
{signal_strength}
        </div>
    </div>
    "##
    )
}

pub fn connection_page(ap_infos: &[AccessPointInfo]) -> String {
    let mut page = PART_1.to_owned();

    for ap_info in ap_infos {
        let wifi_item = &wifi_item(ap_info);
        page = format!("{page}{wifi_item}");
    }

    format!("{page}{PART_2}")
}

//     <div class="wifi-item" data-ssid="HomeNetwork" data-protected="true">
//         <span>HomeNetwork</span>
//         <div class="signal-strength">
//             <div class="signal-bar active"></div>
//             <div class="signal-bar active"></div>
//             <div class="signal-bar active"></div>
//             <div class="signal-bar"></div>
//         </div>
//     </div>

//     <div class="wifi-item" data-ssid="GuestWiFi" data-protected="false">
//         <span>GuestWiFi</span>
//         <div class="signal-strength">
//             <div class="signal-bar active"></div>
//             <div class="signal-bar active"></div>
//             <div class="signal-bar"></div>
//             <div class="signal-bar"></div>
//         </div>
//     </div>
//     <div class="wifi-item" data-ssid="Office_Network" data-protected="true">
//         <span>Office_Network</span>
//         <div class="signal-strength">
//             <div class="signal-bar active"></div>
//             <div class="signal-bar active"></div>
//             <div class="signal-bar active"></div>
//             <div class="signal-bar active"></div>
//         </div>
//     </div>
//     <div class="wifi-item" data-ssid="FreePublicWiFi" data-protected="false">
//         <span>FreePublicWiFi</span>
//         <div class="signal-strength">
//             <div class="signal-bar active"></div>
//             <div class="signal-bar"></div>
//             <div class="signal-bar"></div>
//             <div class="signal-bar"></div>
//         </div>
//     </div>
//     <div class="wifi-item" data-ssid="CafeSpot" data-protected="true">
//         <span>CafeSpot</span>
//         <div class="signal-strength">
//             <div class="signal-bar active"></div>
//             <div class="signal-bar active"></div>
//             <div class="signal-bar"></div>
//             <div class="signal-bar"></div>
//         </div>
//     </div>
