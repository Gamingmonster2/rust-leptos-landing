use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Storage {
    Gb128,
    Gb256,
    Gb512,
    Tb1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ScreenSize {
    Standard61,
    Max67,
}

#[component]
fn App() -> impl IntoView {
    // State for the interactive smartphone customizer
    let (storage, set_storage) = create_signal(Storage::Gb256);
    let (screen, set_screen) = create_signal(ScreenSize::Standard61);
    let (warranty, set_warranty) = create_signal(false);
    let (accessories, set_accessories) = create_signal(false);
    
    // Order Form State
    let (name, set_name) = create_signal(String::new());
    let (phone, set_phone) = create_signal(String::new());
    let (ordered, set_ordered) = create_signal(false);

    // Dynamic Price Calculation Signal
    let total_price = move || {
        let base_price = 899;
        
        let storage_add = match storage.get() {
            Storage::Gb128 => 0,
            Storage::Gb256 => 100,
            Storage::Gb512 => 250,
            Storage::Tb1 => 450,
        };

        let screen_add = match screen.get() {
            ScreenSize::Standard61 => 0,
            ScreenSize::Max67 => 150,
        };

        let warranty_add = if warranty.get() { 99 } else { 0 };
        let acc_add = if accessories.get() { 79 } else { 0 };

        base_price + storage_add + screen_add + warranty_add + acc_add
    };

    // Handle Order Submit
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !name.get().is_empty() && !phone.get().is_empty() {
            set_ordered.set(true);
        }
    };

    view! {
        <div class="app-container">
            // Header Section
            <header class="header-section">
                <div class="logo">{"PHANTOM // X"}</div>
                <div class="badge">{"إصدار محدود 🚀"}</div>
            </header>

            // Hero Section
            <section class="hero-section">
                <h1>{"الجيل القادم من الهواتف الذكية الخارقة"}</h1>
                <p class="subtitle">{"تصميم مستقبلي مدمج بقوة معالج النانو الفائق. متاح الآن للتخصيص الفوري."}</p>
                
                <div class="hero-features">
                    <div class="hero-feature-card">
                        <span class="emoji">"🔋"</span>
                        <h3>"بطارية 72 ساعة"</h3>
                        <p class="monospace">"شحن ذكي بقوة 120 واط"</p>
                    </div>
                    <div class="hero-feature-card">
                        <span class="emoji">"📸"</span>
                        <h3>"كاميرا 200MP"</h3>
                        <p class="monospace">"مستشعر بصري عملاق"</p>
                    </div>
                    <div class="hero-feature-card">
                        <span class="emoji">"⚡"</span>
                        <h3>"معالج Quantum"</h3>
                        <p class="monospace">"بمعمارية 3 نانومتر"</p>
                    </div>
                </div>
            </section>

            // Interactive Configurator Section
            <section class="configurator-section">
                <h2 class="section-title">{"📱 صمم هاتف PHANTOM X الخاص بك"}</h2>
                <p class="section-subtitle">{"اختر المواصفات وشاهد السعر النهائي يتحدث فوراً بفضل تقنية Rust"}</p>

                <div class="config-grid">
                    // Left Column: Controls
                    <div class="config-controls">
                        
                        // Screen Size Selection
                        <div class="control-group">
                            <label class="group-label">{"1. حجم الشاشة الذكية ✨"}</label>
                            <div class="options-row">
                                <button 
                                    class=move || if screen.get() == ScreenSize::Standard61 { "opt-btn active" } else { "opt-btn" }
                                    on:click=move |_| set_screen.set(ScreenSize::Standard61)
                                >
                                    {"شاشة 6.1 بوصة OLED"}
                                    <span class="price-tag">{"+$0"}</span>
                                </button>
                                <button 
                                    class=move || if screen.get() == ScreenSize::Max67 { "opt-btn active" } else { "opt-btn" }
                                    on:click=move |_| set_screen.set(ScreenSize::Max67)
                                >
                                    {"شاشة 6.7 بوصة Pro Max"}
                                    <span class="price-tag">{"+$150"}</span>
                                </button>
                            </div>
                        </div>

                        // Storage Selection
                        <div class="control-group">
                            <label class="group-label">{"2. سعة التخزين الفائقة 💾"}</label>
                            <div class="options-grid">
                                <button 
                                    class=move || if storage.get() == Storage::Gb128 { "opt-btn active" } else { "opt-btn" }
                                    on:click=move |_| set_storage.set(Storage::Gb128)
                                >
                                    {"128 جيجابايت"}
                                    <span class="price-tag">{"+$0"}</span>
                                </button>
                                <button 
                                    class=move || if storage.get() == Storage::Gb256 { "opt-btn active" } else { "opt-btn" }
                                    on:click=move |_| set_storage.set(Storage::Gb256)
                                >
                                    {"256 جيجابايت"}
                                    <span class="price-tag">{"+$100"}</span>
                                </button>
                                <button 
                                    class=move || if storage.get() == Storage::Gb512 { "opt-btn active" } else { "opt-btn" }
                                    on:click=move |_| set_storage.set(Storage::Gb512)
                                >
                                    {"512 جيجابايت"}
                                    <span class="price-tag">{"+$250"}</span>
                                </button>
                                <button 
                                    class=move || if storage.get() == Storage::Tb1 { "opt-btn active" } else { "opt-btn" }
                                    on:click=move |_| set_storage.set(Storage::Tb1)
                                >
                                    {"1 تيرابايت كاملة"}
                                    <span class="price-tag">{"+$450"}</span>
                                </button>
                            </div>
                        </div>

                        // Add-ons / Extras
                        <div class="control-group">
                            <label class="group-label">{"3. الإضافات والترقيات 🛡️"}</label>
                            <div class="checkbox-container" on:click=move |_| set_warranty.update(|v| *v = !*v)>
                                <input type="checkbox" prop:checked=warranty />
                                <div class="checkbox-label">
                                    <span>{"ضمان Phantom Care الممتد (سنتين)"}</span>
                                    <span class="price-tag">{"+$99"}</span>
                                </div>
                            </div>
                            <div class="checkbox-container" on:click=move |_| set_accessories.update(|v| *v = !*v)>
                                <input type="checkbox" prop:checked=accessories />
                                <div class="checkbox-label">
                                    <span>{"حزمة الملحقات السريعة (شاحن 120W + سماعة)"}</span>
                                    <span class="price-tag">{"+$79"}</span>
                                </div>
                            </div>
                        </div>

                    </div>

                    // Right Column: Live Price Display & Checkout
                    <div class="config-summary">
                        <div class="summary-box">
                            <h3>{"ملخص المواصفات المحددة"}</h3>
                            <ul class="summary-list">
                                <li>
                                    <span>{"حجم الشاشة:"}</span>
                                    <span class="monospace">
                                        {move || match screen.get() {
                                            ScreenSize::Standard61 => "6.1\" Super Retina",
                                            ScreenSize::Max67 => "6.7\" Ultra Max",
                                        }}
                                    </span>
                                </li>
                                <li>
                                    <span>{"سعة التخزين:"}</span>
                                    <span class="monospace">
                                        {move || match storage.get() {
                                            Storage::Gb128 => "128 GB",
                                            Storage::Gb256 => "256 GB",
                                            Storage::Gb512 => "512 GB",
                                            Storage::Tb1 => "1 TB",
                                        }}
                                    </span>
                                </li>
                                <li>
                                    <span>{"الضمان الإضافي:"}</span>
                                    <span class="monospace">{move || if warranty.get() { "مفعّل" } else { "غير نشط" }}</span>
                                </li>
                                <li>
                                    <span>{"حزمة الملحقات:"}</span>
                                    <span class="monospace">{move || if accessories.get() { "مضافة" } else { "غير نشطة" }}</span>
                                </li>
                            </ul>

                            <div class="price-display">
                                <span class="price-label">{"السعر الإجمالي التقديري:"}</span>
                                <span class="price-value">"$" {total_price}</span>
                            </div>

                            // Order Form
                            <div class="order-form-container">
                                {move || if ordered.get() {
                                    view! {
                                        <div class="success-message">
                                            <h4>{"🎉 تم تسجيل طلبك بنجاح!"}</h4>
                                            <p>{"سيتصل بك مستشار المبيعات لتأكيد الشحن الفوري خلال 15 دقيقة."}</p>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <form on:submit=handle_submit class="order-form">
                                            <input 
                                                type="text" 
                                                placeholder="الاسم الكامل" 
                                                required 
                                                value=name
                                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                                class="form-input"
                                            />
                                            <input 
                                                type="tel" 
                                                placeholder="رقم الهاتف (مثال: 05xxxxxxxx)" 
                                                required 
                                                value=phone
                                                on:input=move |ev| set_phone.set(event_target_value(&ev))
                                                class="form-input"
                                            />
                                            <button type="submit" class="cta-button">
                                                {"احجز هاتفك الآن وتألق ⚡"}
                                            </button>
                                        </form>
                                    }.into_view()
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Footer
            <footer class="footer">
                <p class="monospace">{"© 2024 PHANTOM MOBILE. ALL RIGHTS RESERVED. POWERED BY RUST + LEPTOS."}</p>
            </footer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
