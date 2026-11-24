const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const source = fs.readFileSync(path.join(__dirname, '..', 'src', 'sidebar.js'), 'utf8');

function load(storage, missing = false) {
    const classes = new Set();
    const events = {};
    const attributes = {};
    const layout = { classList: { toggle(name, enabled) { enabled ? classes.add(name) : classes.delete(name); } } };
    const sidebar = { hidden: false };
    const button = {
        hidden: true,
        textContent: '',
        setAttribute(name, value) { attributes[name] = value; },
        addEventListener(name, callback) { events[name] = callback; },
    };
    const document = {
        querySelector() { return missing ? null : layout; },
        getElementById(id) { return id === 'docs-sidebar' ? sidebar : button; },
    };
    vm.runInNewContext(source, { document, localStorage: storage });
    return { sidebar, button, classes, attributes, click() { events.click(); } };
}

function memoryStorage(initial) {
    const values = { ...initial };
    return {
        values,
        getItem(key) { return values[key] ?? null; },
        setItem(key, value) { values[key] = value; },
    };
}

test('toggles visibility and accessibility state while preserving it across pages', () => {
    const storage = memoryStorage();
    const page = load(storage);
    assert.equal(page.sidebar.hidden, false);
    assert.equal(page.button.hidden, false);
    assert.equal(page.attributes['aria-expanded'], 'true');
    assert.equal(page.button.textContent, '메뉴 접기');
    page.click();
    assert.equal(page.sidebar.hidden, true);
    assert.equal(page.attributes['aria-expanded'], 'false');
    assert.equal(page.button.textContent, '메뉴 펼치기');
    assert.ok(page.classes.has('is-sidebar-collapsed'));
    assert.equal(storage.values['rust-study-sidebar'], 'collapsed');
    const nextPage = load(storage);
    assert.equal(nextPage.sidebar.hidden, true);
    assert.equal(nextPage.attributes['aria-expanded'], 'false');
    nextPage.click();
    assert.equal(nextPage.sidebar.hidden, false);
    assert.equal(storage.values['rust-study-sidebar'], 'expanded');
});

test('uses a visible default for invalid values and handles missing page elements', () => {
    const page = load(memoryStorage({ 'rust-study-sidebar': 'invalid' }));
    assert.equal(page.sidebar.hidden, false);
    assert.doesNotThrow(() => load(memoryStorage(), true));
});

test('still toggles when browser storage is blocked', () => {
    const storage = {
        getItem() { throw new Error('blocked'); },
        setItem() { throw new Error('blocked'); },
    };
    const page = load(storage);
    assert.equal(page.sidebar.hidden, false);
    assert.doesNotThrow(() => page.click());
    assert.equal(page.sidebar.hidden, true);
    assert.equal(page.attributes['aria-expanded'], 'false');
});
