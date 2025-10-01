pub const HTML: &str = r##"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Device Setup</title>
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

        .landing-card {
            background: var(--card-bg);
            border-radius: var(--border-radius);
            padding: 40px;
            box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
            transition: var(--transition);
            display: flex;
            flex-direction: column;
            max-width: 500px;
            margin: 0 auto;
            text-align: center;
        }

        .landing-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 15px 30px rgba(0, 0, 0, 0.3);
        }

        .landing-icon {
            font-size: 4rem;
            margin-bottom: 20px;
            background: linear-gradient(90deg, #4a6cf7, #8a63d2);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        .landing-description {
            color: #a0b3d9;
            margin-bottom: 30px;
            font-size: 1.1rem;
        }

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
            font-size: 1.1rem;
        }

        .landing-btn:hover {
            background: #5b7aff;
            transform: translateY(-2px);
        }

        .wifi-btn {
            background: #333;
        }

        .wifi-btn:hover {
            background: #555;
        }

        .wifi-btn:disabled {
            opacity: 0.7;
            cursor: not-allowed;
            transform: none;
        }

        footer {
            text-align: center;
            margin-top: 50px;
            padding: 20px;
            color: #a0b3d9;
            font-size: 0.9rem;
        }

        /* Popup styles */
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

        @media (max-width: 768px) {
            h1 {
                font-size: 2rem;
            }

            .landing-card {
                padding: 30px 20px;
            }
        }
    </style>
</head>

<body>
    <div class="container">
        <header>
            <h1><i class="fas fa-palette"></i> Device Setup</h1>
            <p class="subtitle">Configure your device settings</p>
        </header>

        <div class="landing-card">
            <div class="landing-icon">
                <i class="fas fa-cogs"></i>
            </div>

            <p class="landing-description">
                Welcome to your device setup. Please choose an option below to continue.
            </p>

            <div class="button-group">
                <button class="landing-btn wifi-btn" id="wifiBtn">
                    <i class="fas fa-wifi"></i> Connect to WiFi
                </button>

                <button class="landing-btn" id="continueBtn">
                    <i class="fas fa-arrow-right"></i> Continue
                </button>
            </div>
        </div>
    </div>

    <!-- Error Popup -->
    <div class="popup-overlay" id="errorPopup">
        <div class="popup error-popup">
            <div class="popup-icon">
                <i class="fas fa-exclamation-triangle"></i>
            </div>
            <h3 class="popup-title">Connection Error</h3>
            <p class="popup-message">Failed to connect to WiFi. Please try again.</p>
            <button class="popup-btn" id="closePopup">OK</button>
        </div>
    </div>

    <footer>
        <p>Device Control Panel © 2025</p>
    </footer>

    <script>
        // Use an IIFE to prevent variable redeclaration issues
        (function() {
            // Get the buttons and popup elements
            let wifiBtn = document.getElementById("wifiBtn");
            let continueBtn = document.getElementById("continueBtn");
            let errorPopup = document.getElementById("errorPopup");
            let closePopup = document.getElementById("closePopup");

            // Store original button text
            let originalWifiText = wifiBtn.innerHTML;

            // Add event listener for the WiFi button
            wifiBtn.addEventListener("click", () => {
                // Change button text and disable it
                wifiBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Searching for networks...';
                wifiBtn.disabled = true;

                fetch('/connection_page')
                    .then(response => {
                        if (!response.ok) {
                            throw new Error('Network response was not ok');
                        }
                        return response.text();
                    })
                    .then(html => {
                        // Replace the current page with the new HTML
                        document.open();
                        document.write(html);
                        document.close();
                    })
                    .catch(err => {
                        console.error("Failed to load WiFi page:", err);

                        // Show error popup
                        errorPopup.classList.add('active');

                        // Reset button after a short delay
                        setTimeout(() => {
                            wifiBtn.innerHTML = originalWifiText;
                            wifiBtn.disabled = false;
                        }, 500);
                    });
            });

            // Add event listener for the Continue button
            continueBtn.addEventListener("click", () => {
                fetch('/color_panel')
                    .then(response => {
                        if (!response.ok) {
                            throw new Error('Network response was not ok');
                        }
                        return response.text();
                    })
                    .then(html => {
                        // Replace the current page with the new HTML
                        document.open();
                        document.write(html);
                        document.close();
                    })
                    .catch(err => {
                        console.error("Failed to load color panel:", err);
                        alert("Failed to load color panel");
                    });
            });

            // Close popup when OK button is clicked
            closePopup.addEventListener("click", () => {
                errorPopup.classList.remove('active');
            });

            // Close popup when clicking outside of it
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
