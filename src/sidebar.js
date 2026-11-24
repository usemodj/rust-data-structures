(() => {
    const layout = document.querySelector('.doc-layout');
    const sidebar = document.getElementById('docs-sidebar');
    const button = document.getElementById('sidebar-toggle');
    if (!layout || !sidebar || !button) return;

    const key = 'rust-study-sidebar';
    let collapsed = false;
    try {
        collapsed = localStorage.getItem(key) === 'collapsed';
    } catch (_) {
        // Storage can be blocked; the button still works for the current page.
    }

    function update() {
        sidebar.hidden = collapsed;
        layout.classList.toggle('is-sidebar-collapsed', collapsed);
        button.setAttribute('aria-expanded', String(!collapsed));
        button.textContent = collapsed ? '메뉴 펼치기' : '메뉴 접기';
    }

    update();
    button.hidden = false;
    button.addEventListener('click', () => {
        collapsed = !collapsed;
        update();
        try {
            localStorage.setItem(key, collapsed ? 'collapsed' : 'expanded');
        } catch (_) {
            // Persistence is optional when storage is unavailable.
        }
    });
})();
