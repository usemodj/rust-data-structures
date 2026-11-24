const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const source = fs.readFileSync(path.join(__dirname, '..', 'src', 'theme.js'), 'utf8');

function load(storage, withSelector = true) {
    const events = {};
    const selectorEvents = {};
    const selector = {
        value: '',
        addEventListener(name, callback) { selectorEvents[name] = callback; },
    };
    const document = {
        documentElement: { dataset: {} },
        addEventListener(name, callback) { events[name] = callback; },
        getElementById() { return withSelector ? selector : null; },
    };
    vm.runInNewContext(source, { document, localStorage: storage });
    return {
        document,
        selector,
        ready() { events.DOMContentLoaded(); },
        change(value) { selector.value = value; selectorEvents.change(); },
    };
}

function memoryStorage(initial) {
    const values = { ...initial };
    return {
        values,
        getItem(key) { return values[key] ?? null; },
        setItem(key, value) { values[key] = value; },
    };
}

test('loads the default and saves the selected theme across page loads', () => {
    const storage = memoryStorage();
    const page = load(storage);
    assert.equal(page.document.documentElement.dataset.theme, 'light');
    page.ready();
    assert.equal(page.selector.value, 'light');
    page.change('dark');
    assert.equal(page.document.documentElement.dataset.theme, 'dark');
    assert.equal(storage.values['rust-study-theme'], 'dark');
    const nextPage = load(storage);
    assert.equal(nextPage.document.documentElement.dataset.theme, 'dark');
    nextPage.ready();
    assert.equal(nextPage.selector.value, 'dark');
    nextPage.change('sepia');
    assert.equal(storage.values['rust-study-theme'], 'sepia');
});

test('ignores unknown theme values', () => {
    const storage = memoryStorage({ 'rust-study-theme': 'unexpected' });
    const page = load(storage);
    assert.equal(page.document.documentElement.dataset.theme, 'light');
    page.ready();
    page.change('unknown');
    assert.equal(page.document.documentElement.dataset.theme, 'light');
    assert.equal(storage.values['rust-study-theme'], 'unexpected');
});

test('continues to work if storage is unavailable or selector is absent', () => {
    const storage = {
        getItem() { throw new Error('blocked'); },
        setItem() { throw new Error('blocked'); },
    };
    const page = load(storage);
    assert.equal(page.document.documentElement.dataset.theme, 'light');
    page.ready();
    assert.doesNotThrow(() => page.change('sepia'));
    assert.equal(page.document.documentElement.dataset.theme, 'sepia');
    assert.doesNotThrow(() => load(storage, false).ready());
});
