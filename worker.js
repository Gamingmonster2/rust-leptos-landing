export default {
  async fetch(request, env, ctx) {
    // الواجهة الاحترافية لمتجر الفانتوم للهواتف
    const htmlContent = `
<!DOCTYPE html>
<html lang="ar" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Phantom Store | متجر الفانتوم للهواتف</title>
    <style>
        :root {
            --bg-dark: #0f172a;
            --card-bg: #1e293b;
            --accent-blue: #38bdf8;
            --text-main: #f8fafc;
            --text-muted: #94a3b8;
            --border-color: #334155;
        }
        body {
            font-family: system-ui, -apple-system, sans-serif;
            background-color: var(--bg-dark);
            color: var(--text-main);
            margin: 0;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
        }
        .app-container {
            background-color: var(--card-bg);
            border: 1px solid var(--border-color);
            border-radius: 16px;
            padding: 32px;
            width: 100%;
            max-width: 400px;
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
        }
        .header { text-align: center; margin-bottom: 25px; }
        .header h1 { margin: 0; color: var(--accent-blue); font-size: 26px; }
        .header p { margin: 5px 0 0 0; color: var(--text-muted); font-size: 14px; }
        .option-group { margin-bottom: 20px; }
        .option-group label { display: block; margin-bottom: 8px; font-size: 14px; font-weight: 600; }
        select {
            width: 100%;
            padding: 12px;
            background-color: var(--bg-dark);
            color: var(--text-main);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            font-size: 15px;
            outline: none;
            cursor: pointer;
        }
        .price-display-box {
            background: rgba(56, 189, 248, 0.08);
            border: 1px solid var(--accent-blue);
            border-radius: 10px;
            padding: 15px;
            text-align: center;
            margin-top: 25px;
        }
        .price-amount { font-size: 30px; font-weight: 800; color: var(--accent-blue); }
    </style>
</head>
<body>
    <div class="app-container">
        <div class="header">
            <h1>📱 Phantom Store</h1>
            <p>منصة تخصيص الهواتف وحساب الأسعار</p>
        </div>
        <div class="option-group">
            <label>السعة التخزينية</label>
            <select id="storage" onchange="calculate()">
                <option value="0">128 جيجابايت (الأساسي)</option>
                <option value="100">256 جيجابايت (+100 $)</option>
                <option value="250">512 جيجابايت (+250 $)</option>
            </select>
        </div>
        <div class="option-group">
            <label>مقاس الشاشة</label>
            <select id="screen" onchange="calculate()">
                <option value="0">6.1 بوصة القياسي</option>
                <option value="150">6.7 بوصة المطور (+150 $)</option>
            </select>
        </div>
        <div class="price-display-box">
            <div class="price-amount">$<span id="price">799</span></div>
        </div>
    </div>
    <script>
        function calculate() {
            const base = 799;
            const storage = parseInt(document.getElementById('storage').value);
            const screen = parseInt(document.getElementById('screen').value);
            document.getElementById('price').innerText = base + storage + screen;
        }
    </script>
</body>
</html>
`;

    return new Response(htmlContent, {
      headers: { "content-type": "text/html;charset=UTF-8" },
    });
  }
};
