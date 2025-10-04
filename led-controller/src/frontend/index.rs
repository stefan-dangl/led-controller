use crate::frontend::COMMON_HEADER;

const HTML: &str = r##"
        .button-group {
            display: flex;
            flex-direction: column;
            gap: 15px;
            margin-top: 20px;
        }

        .landing-btn {
            background: var(--primary);
            color: white;
            border: none;
            border-radius: 8px;
            padding: 15px 20px;
            font-weight: 600;
            cursor: pointer;
            transition: var(--transition);
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 10px;
            font-size: var(--medium-font-size);
        }

        .landing-btn:hover {
            background: #5b7aff;
            transform: translateY(-2px);
        }

        .wifi-btn {
            background: var(--dark-gray);
        }

        .wifi-btn:hover {
            background: var(--light-gray);
        }

        .wifi-btn:disabled {
            opacity: 0.7;
            cursor: not-allowed;
            transform: none;
        }

        .popup-overlay {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: rgba(0, 0, 0, 0.7);
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 1000;
            opacity: 0;
            visibility: hidden;
            transition: var(--transition);
        }

        .popup-overlay.active {
            opacity: 1;
            visibility: visible;
        }

        .popup {
            background: var(--card-bg);
            border-radius: var(--border-radius);
            padding: 30px;
            box-shadow: 0 15px 30px rgba(0, 0, 0, 0.4);
            max-width: 400px;
            width: 90%;
            text-align: center;
            transform: translateY(-20px);
            transition: var(--transition);
        }

        .popup-overlay.active .popup {
            transform: translateY(0);
        }

        .popup-icon {
            font-size: 3rem;
            margin-bottom: 15px;
        }

        .popup-title {
            font-size: 1.5rem;
            margin-bottom: 10px;
        }

        .popup-message {
            color: #a0b3d9;
            margin-bottom: 20px;
        }

        .popup-btn {
            background: var(--primary);
            color: white;
            border: none;
            border-radius: 8px;
            padding: 10px 20px;
            font-weight: 600;
            cursor: pointer;
            transition: var(--transition);
        }

        .popup-btn:hover {
            background: #5b7aff;
        }

        .error-popup .popup-icon {
            color: var(--error);
        }

        .error-popup .popup-title {
            color: var(--error);
        }

        /* Landing specific*/

        @media (max-width: 768px) {
            h1 {
                font-size: 2rem;
            }

            .slim-card {
                padding: 30px 20px;
            }
        }
    </style>
</head>

<body>
    <div class="container">
        <header>
            <h1>Color Control Panel</h1>
            <p class="subtitle">Connect to your WiFi first or directly choose a color.</p>
        </header>

        <div class="slim-card">
            <div class="card-headline">
                Welcome
            </div>

            <div class="button-group">
                <button class="landing-btn wifi-btn" id="wifiBtn">
                    Connect to WiFi
                </button>

                <button class="landing-btn" id="continueBtn">
                    Choose Color
                </button>
            </div>
        </div>
    </div>

    <div class="popup-overlay" id="errorPopup">
        <div class="popup error-popup">
            <h3 class="popup-title">Connection Error</h3>
            <p class="popup-message">Failed to connect to WiFi. Please try again.</p>
            <button class="popup-btn" id="closePopup">OK</button>
        </div>
    </div>

    <footer>
        <p>Stefan Dangl © 2025</p>
    </footer>

    <script>
        (function () {
            let wifiBtn = document.getElementById("wifiBtn");
            let continueBtn = document.getElementById("continueBtn");
            let errorPopup = document.getElementById("errorPopup");
            let closePopup = document.getElementById("closePopup");

            let originalWifiText = wifiBtn.innerHTML;

            wifiBtn.addEventListener("click", () => {
                wifiBtn.innerHTML = 'Searching for networks...';
                wifiBtn.disabled = true;

                fetch('/connection_page')
                    .then(response => {
                        if (!response.ok) {
                            throw new Error('Network response was not ok');
                        }
                        return response.text();
                    })
                    .then(html => {
                        document.open();
                        document.write(html);
                        document.close();
                    })
                    .catch(err => {
                        console.error("Failed to load WiFi page:", err);

                        errorPopup.classList.add('active');

                        setTimeout(() => {
                            wifiBtn.innerHTML = originalWifiText;
                            wifiBtn.disabled = false;
                        }, 500);
                    });
            });

            continueBtn.addEventListener("click", () => {
                fetch('/color_panel')
                    .then(response => {
                        if (!response.ok) {
                            throw new Error('Network response was not ok');
                        }
                        return response.text();
                    })
                    .then(html => {
                        document.open();
                        document.write(html);
                        document.close();
                    })
                    .catch(err => {
                        console.error("Failed to load color panel:", err);
                        alert("Failed to load color panel");
                    });
            });

            closePopup.addEventListener("click", () => {
                errorPopup.classList.remove('active');
            });

            errorPopup.addEventListener("click", (e) => {
                if (e.target === errorPopup) {
                    errorPopup.classList.remove('active');
                }
            });
        })();
    </script>
</body>

</html>
"##;

pub fn index() -> String {
    format!("{COMMON_HEADER}{HTML}")
}
