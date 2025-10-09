use crate::frontend::COMMON_HEADER;

const HTML: &str = r##"
        .color-section {
            margin-bottom: 20px;
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
            font-size: var(--medium-font-size);
        }

        .set-color-btn:hover {
            background: #5b7aff;
            transform: translateY(-2px);
        }


        .effect-buttons {
            display: flex;
            gap: 10px;
            margin-top: 15px;
        }

        .effect-btn {
            flex: 1;
            border: none;
            border-radius: 8px;
            font-weight: 600;
            cursor: pointer;
            transition: var(--transition);
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            height: 80px;
            width: 60px;
            padding: 0;
            font-size: var(--medium-font-size);
            text-align: center;
            line-height: 1.2;
        }

        .rainbow-btn {
            background: var(--dark-gray);
        }

        .rainbow-btn:hover {
            background: var(--light-gray);
            transform: translateY(-2px);
        }

        .off-btn {
            background: var(--dark-gray);
        }

        .off-btn:hover {
            background: var(--light-gray);
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

        /* ... Color Panel specific */

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
            <p class="subtitle">Please choose a color</p>
        </header>

        <div id="device" class="slim-card">
            <div class="card-headline">
                Device
                <span class="status-indicator status-connected"></span>
            </div>

            <div class="color-section">
                <div class="color-input-group">
                    <input type="color" class="color-input" value="#00a0a0">
                    <button class="set-color-btn">
                        Set Color
                    </button>
                </div>

                <div class="effect-buttons">
                    <button class="effect-btn rainbow-btn" id="rainbowBtn">
                        Rainbow
                    </button>
                    <button class="effect-btn off-btn" id="offBtn">
                        Off
                    </button>
                </div>
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
            let setColorBtn = document.querySelector(".set-color-btn");
            let colorInput = document.querySelector("input[type='color']");
            let rainbowBtn = document.getElementById("rainbowBtn");
            let offBtn = document.getElementById("offBtn");
            let backBtn = document.getElementById("backBtn");

            let apiAddress = `${window.location.protocol}//${window.location.hostname}${window.location.port ? ':' + window.location.port : ''}`;

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

pub fn color_panel() -> String {
    format!("{COMMON_HEADER}{HTML}")
}
