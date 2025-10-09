pub mod color_panel;
pub mod index;
pub mod wifi_connection;

const COMMON_HEADER: &str = r##"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Color Control Panel</title>
    <style>
        :root {
            --primary: #4a6cf7;
            --dark: #1d2a3a;
            --darker: #131a25;
            --light: #f5f8ff;
            --card-bg: #243247;
            --success: #00c896;
            --error: #ff4757;
            --dark-gray: #333;
            --light-gray: #444;
            --border-radius: 12px;
            --transition: all 0.3s ease;
            --header-font-size: 2.5rem;
            --large-font-size: 1.5rem;
            --medium-font-size: 1.1rem;
            --small-font-size: 0.9rem;
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
            font-size: var(--header-font-size);
            margin-bottom: 10px;
            background: linear-gradient(90deg, #4a6cf7, #8a63d2);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        .subtitle {
            color: #a0b3d9;
            font-size: var(--medium-font-size);
        }

        .card-headline {
            font-size: var(--large-font-size);
            font-weight: 600;
            margin-bottom: 15px;
            color: #fff;
            display: flex;
            align-items: center;
            gap: 10px;
        }

        .card-headline i {
            color: var(--primary);
        }

        footer {
            text-align: center;
            margin-top: 50px;
            padding: 20px;
            color: #a0b3d9;
            font-size: var(--small-font-size);
        }

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
            font-size: var(--medium-font-size);
        }

        .back-btn:hover {
            background: #5b7aff;
            transform: translateY(-2px);
        }

        .slim-card {
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

        .slim-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 15px 30px rgba(0, 0, 0, 0.3);
        }
"##;
