// Service worker so the app installs and works offline.
//
// Pages are fetched from the network first so new releases show up, falling
// back to the cache offline. Trunk gives the JS and wasm hashed names, so they
// are cached as they are fetched, and older builds are dropped from the cache.

const CACHE = "tuble-v1";
const PRECACHE = [
    "./",
    "manifest.webmanifest",
    "icons/favicon.svg",
    "icons/icon-192.png",
    "icons/icon-512.png",
    "icons/icon-maskable-512.png",
    "icons/apple-touch-icon.png",
];
// Matches trunk's hashed output, e.g. tuble-<hash>_bg.wasm or style-<hash>.css,
// capturing the name and extension that stay the same between builds.
const BUILD_FILE = /\/([\w-]+)-[0-9a-f]{16}((?:_bg)?\.\w+)$/;

self.addEventListener("install", (event) => {
    event.waitUntil(caches.open(CACHE).then((cache) => cache.addAll(PRECACHE)));
    self.skipWaiting();
});

self.addEventListener("activate", (event) => {
    event.waitUntil(
        caches
            .keys()
            .then((keys) => Promise.all(keys.filter((key) => key !== CACHE).map((key) => caches.delete(key))))
            .then(() => self.clients.claim()),
    );
});

self.addEventListener("fetch", (event) => {
    const { request } = event;
    const url = new URL(request.url);
    if (request.method !== "GET" || url.origin !== self.location.origin) {
        return;
    }
    if (request.mode === "navigate") {
        event.respondWith(networkFirst(request));
    } else {
        event.respondWith(cacheFirst(request, url));
    }
});

async function networkFirst(request) {
    const cache = await caches.open(CACHE);
    try {
        const response = await fetch(request);
        if (response.ok) {
            await cache.put("./", response.clone());
        }
        return response;
    } catch (error) {
        return (await cache.match("./")) ?? Response.error();
    }
}

async function cacheFirst(request, url) {
    const cache = await caches.open(CACHE);
    const cached = await cache.match(request);
    if (cached) {
        return cached;
    }
    const response = await fetch(request);
    if (response.ok) {
        const build = url.pathname.match(BUILD_FILE);
        if (build) {
            // Drop the same kind of file from earlier builds.
            for (const key of await cache.keys()) {
                const old = new URL(key.url).pathname.match(BUILD_FILE);
                if (old && old[1] === build[1] && old[2] === build[2] && key.url !== request.url) {
                    await cache.delete(key);
                }
            }
        }
        await cache.put(request, response.clone());
    }
    return response;
}
