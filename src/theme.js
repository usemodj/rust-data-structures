(() => {
    const key = 'rust-study-theme';
    const themes = new Set(['light', 'dark', 'sepia']);
    let theme = 'light';
    try {
        const saved = localStorage.getItem(key);
        if (themes.has(saved)) theme = saved;
    } catch (_) {
        // Storage can be blocked; the theme still works for the current page.
    }
    document.documentElement.dataset.theme = theme;

    document.addEventListener('DOMContentLoaded', () => {
        const selector = document.getElementById('theme-select');
        if (!selector) return;
        selector.value = theme;
        selector.addEventListener('change', () => {
            if (!themes.has(selector.value)) return;
            theme = selector.value;
            document.documentElement.dataset.theme = theme;
            try {
                localStorage.setItem(key, theme);
            } catch (_) {
                // Persistence is optional when storage is unavailable.
            }
        });
    });
})();
