const CACHE_PREFIX = "origin-os-shell-runtime-";
const CACHE_NAME = `${CACHE_PREFIX}v2`;

self.addEventListener("install", (event) => {
  self.skipWaiting();

  const scopeUrl = new URL(self.registration.scope);
  const shellAssets = [
    scopeUrl.toString(),
    new URL("manifest.webmanifest", scopeUrl).toString(),
  ];

  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(shellAssets))
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) =>
      Promise.all(
        cacheNames
          .filter((cacheName) => cacheName.startsWith(CACHE_PREFIX) && cacheName !== CACHE_NAME)
          .map((cacheName) => caches.delete(cacheName))
      )
    ).then(() => self.clients.claim())
  );
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET" || event.request.mode !== "navigate") {
    return;
  }

  event.respondWith(
    fetch(event.request).then((response) => {
      const responseClone = response.clone();
      event.waitUntil(
        caches.open(CACHE_NAME).then((cache) => cache.put(event.request, responseClone))
      );
      return response;
    }).catch(async () => {
      const cached = await caches.match(event.request);
      if (cached) {
        return cached;
      }

      return caches.match(self.registration.scope);
    })
  );
});
