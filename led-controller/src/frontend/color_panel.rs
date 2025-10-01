pub const HTML: &str = r##"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Device Color Control</title>
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

        .device-card {
            background: var(--card-bg);
            border-radius: var(--border-radius);
            padding: 20px;
            box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
            transition: var(--transition);
            display: flex;
            flex-direction: column;
            max-width: 400px;
            margin: 0 auto;
        }

        .device-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 15px 30px rgba(0, 0, 0, 0.3);
        }

        .device-name {
            font-size: 1.3rem;
            font-weight: 600;
            margin-bottom: 15px;
            color: #fff;
            display: flex;
            align-items: center;
            gap: 10px;
        }

        .device-name i {
            color: var(--primary);
        }

        .color-section {
            margin-bottom: 20px;
        }

        .color-label {
            display: block;
            margin-bottom: 8px;
            font-size: 0.9rem;
            color: #a0b3d9;
        }

        .color-input-group {
            display: flex;
            gap: 10px;
            margin-top: 15px;
        }

        .color-input {
            flex: 1;
            height: 42px;
            border: none;
            border-radius: 8px;
            padding: 0;
            cursor: pointer;
            background: transparent;
        }

        .color-input::-webkit-color-swatch {
            border: none;
            border-radius: 6px;
        }

        .color-input::-moz-color-swatch {
            border: none;
            border-radius: 6px;
        }

        .set-color-btn {
            background: var(--primary);
            color: white;
            border: none;
            border-radius: 8px;
            padding: 0 20px;
            font-weight: 600;
            cursor: pointer;
            transition: var(--transition);
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .set-color-btn:hover {
            background: #5b7aff;
            transform: translateY(-2px);
        }

        /* New styles for the rainbow and off buttons */
        .effect-buttons {
            display: flex;
            gap: 10px;
            margin-top: 15px;
        }

        .effect-btn {
            flex: 1;
            height: 42px;
            border: none;
            border-radius: 8px;
            font-weight: 600;
            cursor: pointer;
            transition: var(--transition);
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            color: white;
        }

        .rainbow-btn {
            background: #333;
        }

        .rainbow-btn i {
            background: linear-gradient(90deg, #ff0000, #ff9900, #ffff00, #00ff00, #00ffff, #0000ff, #9900ff);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .rainbow-btn:hover {
            background: #555;
            transform: translateY(-2px);
        }

        .off-btn {
            background: #333;
        }

        .off-btn:hover {
            background: #555;
            transform: translateY(-2px);
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

        /* Back button styles - now matching the rainbow button width */
        .back-btn-container {
            display: flex;
            justify-content: center;
            margin: 30px auto;
            max-width: 400px;
        }

        .back-btn {
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
            width: calc(50% - 5px);
            /* Same width as rainbow button */
        }

        .back-btn:hover {
            background: #5b7aff;
            transform: translateY(-2px);
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
            <h1><i class="fas fa-palette"></i> Color Control Panel</h1>
            <p class="subtitle">Control your LED color</p>
        </header>

        <div id="device" class="device-card">
            <div class="device-name">
                <i class="fas fa-microchip"></i> MCU Device
                <span class="status-indicator status-connected"></span>
            </div>

            <div class="color-section">
                <span class="color-label">Set Color</span>
                <div class="color-input-group">
                    <input type="color" class="color-input" value="#ffffff">
                    <button class="set-color-btn">
                        <i class="fas fa-paint-brush"></i> Set Color
                    </button>
                </div>

                <!-- New effect buttons section -->
                <div class="effect-buttons">
                    <button class="effect-btn rainbow-btn" id="rainbowBtn">
                        <i class="fas fa-rainbow"></i> Rainbow
                    </button>
                    <button class="effect-btn off-btn" id="offBtn">
                        <i class="fas fa-power-off"></i> Off
                    </button>
                </div>
            </div>
        </div>

        <!-- Back button positioned between device card and footer -->
        <div class="back-btn-container">
            <button class="back-btn" id="backBtn">
                <i class="fas fa-arrow-left"></i> Back
            </button>
        </div>
    </div>

    <footer>
        <p>Color Control Panel © 2025</p>
    </footer>

    <script>
        (function() {
            // Get the set color button and color input
            let setColorBtn = document.querySelector(".set-color-btn");
            let colorInput = document.querySelector("input[type='color']");
            let rainbowBtn = document.getElementById("rainbowBtn");
            let offBtn = document.getElementById("offBtn");
            let backBtn = document.getElementById("backBtn");

            // Set the API endpoint
            let apiAddress = `${window.location.protocol}//${window.location.hostname}${window.location.port ? ':' + window.location.port : ''}`;

            // Add event listener for the set color button
            setColorBtn.addEventListener("click", () => {
                let newColor = colorInput.value.replace("#", "").toLowerCase();

                fetch(`${apiAddress}/set_color`, {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({
                        color: newColor
                    })
                }).then(response => {
                    if (!response.ok) {
                        alert("Failed to set color");
                    } else {
                        console.log("Color set successfully");
                    }
                }).catch(err => {
                    console.error("Network error:", err);
                    alert("Failed to set color");
                });
            });

            // Add event listener for the rainbow button
            rainbowBtn.addEventListener("click", () => {
                fetch(`${apiAddress}/rainbow`, {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    }
                }).then(response => {
                    if (!response.ok) {
                        alert("Failed to activate rainbow effect");
                    } else {
                        console.log("Rainbow effect activated");
                        // Add a subtle animation to confirm activation
                        rainbowBtn.style.transform = "scale(0.95)";
                        setTimeout(() => {
                            rainbowBtn.style.transform = "";
                        }, 150);
                    }
                }).catch(err => {
                    console.error("Network error:", err);
                    alert("Failed to activate rainbow effect");
                });
            });

            // Add event listener for the off button
            offBtn.addEventListener("click", () => {
                fetch(`${apiAddress}/set_color`, {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({
                        color: "000000"
                    })
                }).then(response => {
                    if (!response.ok) {
                        alert("Failed to turn off");
                    } else {
                        console.log("Device turned off");
                        // Add a subtle animation to confirm activation
                        offBtn.style.transform = "scale(0.95)";
                        setTimeout(() => {
                            offBtn.style.transform = "";
                        }, 150);
                    }
                }).catch(err => {
                    console.error("Network error:", err);
                    alert("Failed to turn off");
                });
            });

            // Add event listener for the back button
            backBtn.addEventListener("click", () => {
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
                    // Replace the entire document with the new HTML
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
