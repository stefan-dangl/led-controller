use crate::frontend::COMMON_HEADER;
use esp_idf_svc::wifi::AccessPointInfo;

const HTML_1: &str = r##"
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
            font-size: var(--medium-font-size);
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

        .status-connected {
            background-color: var(--success);
        }

        .success-ip-message {
            margin-top: 15px;
            padding: 10px;
            border-radius: 8px;
            text-align: center;
            background: rgba(0, 200, 150, 0.2);
            color: var(--success);
            display: none;
        }

        .success-ip-message.active {
            display: block;
        }

        .ip-address {
            color: var(--success);
            text-decoration: underline;
            cursor: pointer;
            font-weight: 600;
        }

        .ip-address:hover {
            color: #00e6a8;
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
            <h1>Color Control Panel</h1>
            <p class="subtitle">Connect to your WiFi</p>
        </header>

        <div class="slim-card">
            <div class="card-headline">
                Available Networks
            </div>

            <div class="wifi-list">
"##;

const HTML_2: &str = r##"
            </div>

            <div class="password-section" id="passwordSection">
                <span class="password-label">Enter Password for <span id="selectedSsid"></span></span>
                <input type="password" class="password-input" id="passwordInput" placeholder="Password">
                <button class="connect-btn" id="connectBtn">
                    Connect
                </button>
            </div>

            <div class="status-message" id="statusMessage"></div>

            <div class="success-ip-message" id="successIpMessage">
                Please continue at <span class="ip-address" id="ipAddress"></span>
            </div>
        </div>

        <div class="back-btn-container">
            <button class="back-btn" id="backBtn">
                Back
            </button>
        </div>
    </div>

    <footer>
        <p>Stefan Dangl © 2025</p>
    </footer>

    <script>
        (function () {
            let wifiItems = document.querySelectorAll('.wifi-item');
            let passwordSection = document.getElementById('passwordSection');
            let selectedSsid = document.getElementById('selectedSsid');
            let passwordInput = document.getElementById('passwordInput');
            let connectBtn = document.getElementById('connectBtn');
            let statusMessage = document.getElementById('statusMessage');
            let successIpMessage = document.getElementById('successIpMessage');
            let ipAddress = document.getElementById('ipAddress');
            let backBtn = document.getElementById('backBtn');

            let selectedWifi = null;

            wifiItems.forEach(item => {
                item.addEventListener('click', () => {
                    wifiItems.forEach(i => i.classList.remove('selected'));

                    item.classList.add('selected');

                    selectedWifi = {
                        ssid: item.dataset.ssid,
                        protected: item.dataset.protected === 'true'
                    };

                    if (selectedWifi.protected) {
                        selectedSsid.textContent = selectedWifi.ssid;
                        passwordSection.classList.add('active');
                        passwordInput.value = '';
                        passwordInput.focus();
                        statusMessage.className = 'status-message';
                        successIpMessage.classList.remove('active');
                    } else {
                        passwordSection.classList.remove('active');
                        connectToWifi(selectedWifi.ssid);
                    }
                });
            });

            connectBtn.addEventListener('click', () => {
                if (!selectedWifi) {
                    showStatus('Please select a WiFi network first', 'error');
                    return;
                }

                if (selectedWifi.protected) {
                    let password = passwordInput.value.trim();
                    if (!password) {
                        showStatus('Please enter a password', 'error');
                        return;
                    }
                    connectToWifi(selectedWifi.ssid, password);
                } else {
                    connectToWifi(selectedWifi.ssid);
                }
            });

            passwordInput.addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    connectBtn.click();
                }
            });

            function connectToWifi(ssid, password = null) {
                let requestData = { ssid };
                if (password) {
                    requestData.password = password;
                }

                showStatus('Connecting to ' + ssid + '...', 'connecting');

                fetch('/connect_to_wifi', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(requestData)
                })
                    .then(response => {

                        console.log("response:");
                        console.log(response);

                        if (!response.ok) {
                            throw new Error('Network response was not ok');
                        }
                        console.log("response to json");
                        let response_json = response.json();
                        return response_json;
                    })
                    .then(data => {
                        showStatus('Successfully connected to ' + ssid, 'success');

                        if (data.ip_address) {
                            showSuccessIpMessage(data.ip_address);
                        }

                        setTimeout(() => {
                            wifiItems.forEach(i => i.classList.remove('selected'));
                            passwordSection.classList.remove('active');
                            selectedWifi = null;
                            statusMessage.className = 'status-message';
                        }, 3000);
                    })
                    .catch(error => {
                        console.error('Error connecting to WiFi:', error);
                        showStatus('Failed to connect to ' + ssid, 'error');
                    });
            }

            function showStatus(message, type) {
                statusMessage.textContent = message;
                statusMessage.className = 'status-message ' + type;
            }

            function showSuccessIpMessage(ip) {
                ipAddress.textContent = ip;
                successIpMessage.classList.add('active');

                ipAddress.onclick = function () {
                    window.location.href = 'http://' + ip;
                };
            }

            backBtn.addEventListener('click', () => {
                fetch("/", {
                    method: "GET"
                })
                    .then(response => {
                        if (!response.ok) {
                            throw new Error("Failed to fetch new content");
                        }
                        return response.text();
                    })
                    .then(html => {
                        document.open();
                        document.write(html);
                        document.close();
                    })
                    .catch(err => {
                        console.error("Network error:", err);
                        alert("Failed to go back");
                    });
            });
        })();
    </script>
</body>

</html>
"##;

const NUMBER_OF_BARS: i8 = 4;
const ACTIVE_BAR: &str = r##"
<div class="signal-bar active"></div>
"##;
const INACTIVE_BAR: &str = r##"
<div class="signal-bar"></div>
"##;

pub fn number_of_active_wifi_bars(signal_strength: i8, number_of_bars: i8) -> i8 {
    let inactive_bars = signal_strength / (i8::MIN / number_of_bars);
    number_of_bars - inactive_bars
}

fn signal_strength(signal_strength: i8) -> String {
    let active_bars = number_of_active_wifi_bars(signal_strength, NUMBER_OF_BARS);

    let mut output = String::new();

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
    let is_protected = ap_info.auth_method.is_some();

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
    let mut page = format!("{COMMON_HEADER}{HTML_1}");

    for ap_info in ap_infos {
        let wifi_item = &wifi_item(ap_info);
        page = format!("{page}{wifi_item}");
    }

    format!("{page}{HTML_2}")
}
