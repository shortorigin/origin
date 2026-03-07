self.addEventListener("install", (event) => {
  const scopeUrl = new URL(self.registration.scope);
  event.waitUntil(
    caches.open("origin-os-shell-v1").then((cache) =>
      cache.addAll([
        scopeUrl.toString(),
        new URL("manifest.webmanifest", scopeUrl).toString(),
      ])
    )
  );
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET") {
    return;
  }

  event.respondWith(
    caches.match(event.request).then((cached) => cached || fetch(event.request))
  );
});
