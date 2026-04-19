function setStatus(text) {
  document.getElementById("status").textContent = text;
}

function appendOutput(text) {
  const out = document.getElementById("output");
  out.textContent += text;
  out.scrollTop = out.scrollHeight;
}

function clearOutput() {
  document.getElementById("output").textContent = "";
}

async function fetchBytes(url, { optional = false } = {}) {
  const res = await fetch(url);
  if (!res.ok) {
    if (optional && res.status === 404) {
      return null;
    }
    throw new Error(`Failed to fetch ${url}: ${res.status} ${res.statusText}`);
  }
  return new Uint8Array(await res.arrayBuffer());
}

async function loadAssetManifest() {
  // Copied to dist via <link data-trunk rel="copy-dir" href="./assets" />
  // in web/index.html.
  const url = new URL("./assets/manifest.json", import.meta.url);
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(
      `Failed to fetch assets/manifest.json: ${res.status} ${res.statusText}`,
    );
  }
  return await res.json();
}

async function loadEpheAssetsFromManifest() {
  const manifest = await loadAssetManifest();

  const mount = typeof manifest.mount === "string" ? manifest.mount : "ephe";
  const files = Array.isArray(manifest.files) ? manifest.files : [];

  const loadedFiles = [];

  for (const f of files) {
    if (!f || typeof f.path !== "string") continue;
    const optional = !!f.optional;

    const fileUrl = new URL(`./assets/${f.path}`, import.meta.url);
    const bytes = await fetchBytes(fileUrl, { optional });
    if (bytes === null) {
      // Optional asset missing; keep going.
      console.warn(`[assets] Missing optional asset: ${f.path}`);
      continue;
    }

    loadedFiles.push({ name: f.path, bytes });
  }

  return { mount, loadedFiles };
}

const PLANETS = [
  // Note: Swiss Ephemeris uses different data files for different bodies.
  // We treat these as optional web assets, so the demo can still run even if
  // some files are missing.
  { name: "Sun", getIpl: (e) => e.swisseph_se_sun(), requires: "seas_18.se1" },
  { name: "Moon", getIpl: (e) => e.swisseph_se_moon(), requires: "semo_18.se1" },
  {
    name: "Mercury",
    getIpl: (e) => e.swisseph_se_mercury(),
    requires: "seas_18.se1",
  },
  { name: "Venus", getIpl: (e) => e.swisseph_se_venus(), requires: "seas_18.se1" },
  { name: "Mars", getIpl: (e) => e.swisseph_se_mars(), requires: "seas_18.se1" },
  {
    name: "Jupiter",
    getIpl: (e) => e.swisseph_se_jupiter(),
    requires: "sepl_18.se1",
  },
  {
    name: "Saturn",
    getIpl: (e) => e.swisseph_se_saturn(),
    requires: "sepl_18.se1",
  },
  {
    name: "Uranus",
    getIpl: (e) => e.swisseph_se_uranus(),
    requires: "sepl_18.se1",
  },
  {
    name: "Neptune",
    getIpl: (e) => e.swisseph_se_neptune(),
    requires: "sepl_18.se1",
  },
  { name: "Pluto", getIpl: (e) => e.swisseph_se_pluto(), requires: "sepl_18.se1" },
];

function setDateDefaults() {
  const dateEl = document.getElementById("date");
  if (!dateEl.value) {
    const now = new Date();
    // Use UTC date portion.
    const yyyy = now.getUTCFullYear();
    const mm = String(now.getUTCMonth() + 1).padStart(2, "0");
    const dd = String(now.getUTCDate()).padStart(2, "0");
    dateEl.value = `${yyyy}-${mm}-${dd}`;

    document.getElementById("hour").value = String(now.getUTCHours());
    document.getElementById("minute").value = String(now.getUTCMinutes());
    document.getElementById("second").value = String(now.getUTCSeconds());
  }
}

function parseUtcFromInputs() {
  const dateStr = document.getElementById("date").value;
  if (!dateStr) {
    throw new Error("Date is required");
  }
  const [y, m, d] = dateStr.split("-").map((n) => Number(n));
  if (!Number.isFinite(y) || !Number.isFinite(m) || !Number.isFinite(d)) {
    throw new Error("Invalid date");
  }

  const hour = Number(document.getElementById("hour").value || 0);
  const minute = Number(document.getElementById("minute").value || 0);
  const second = Number(document.getElementById("second").value || 0);

  const hourUt = hour + minute / 60 + second / 3600;
  return { year: y, month: m, day: d, hourUt };
}

function renderResults(rows) {
  const tbody = document.getElementById("results");
  tbody.textContent = "";

  for (const row of rows) {
    const tr = document.createElement("tr");
    const cols = [
      row.body,
      row.lon,
      row.lat,
      row.dist,
      row.speedLon,
      row.speedLat,
      row.speedDist,
    ];
    for (const c of cols) {
      const td = document.createElement("td");
      td.textContent = c;
      tr.appendChild(td);
    }
    tbody.appendChild(tr);
  }
}

function readLastError(instance) {
  const e = instance.exports;
  if (typeof e.swisseph_last_error_ptr !== "function") {
    return "";
  }

  const ptr = e.swisseph_last_error_ptr();
  if (!ptr) {
    return "";
  }

  const mem = new Uint8Array(e.memory.buffer);
  // NUL-terminated.
  let end = ptr;
  while (end < mem.length && mem[end] !== 0) end++;
  return new TextDecoder().decode(mem.slice(ptr, end));
}

function writeUtf8ToWasm(instance, str) {
  const e = instance.exports;
  const enc = new TextEncoder();
  const bytes = enc.encode(str);
  const ptr = e.swisseph_alloc(bytes.length);
  const mem = new Uint8Array(e.memory.buffer);
  mem.set(bytes, ptr);
  return { ptr, len: bytes.length };
}

function freeUtf8FromWasm(instance, { ptr, len }) {
  instance.exports.swisseph_dealloc(ptr, len);
}

function allocF64(instance, len) {
  const ptr = instance.exports.swisseph_alloc_f64(len);
  return { ptr, len };
}

function freeF64(instance, { ptr, len }) {
  instance.exports.swisseph_dealloc_f64(ptr, len);
}

async function loadUnknownInstance() {
  clearOutput();
  setStatus("Loading wasm...");

  // The wasm module is built and copied into this folder automatically by a
  // Trunk `pre_build` hook (see web/Trunk.toml).
  const wasmUrl = new URL("./swisseph_unknown.wasm", import.meta.url);
  const wasmResponse = await fetch(wasmUrl);
  if (!wasmResponse.ok) {
    throw new Error(
      `Failed to fetch swisseph_unknown.wasm: ${wasmResponse.status} ${wasmResponse.statusText}`,
    );
  }

  // Prefer compileStreaming when possible.
  let module;
  try {
    module = await WebAssembly.compileStreaming(wasmResponse);
  } catch {
    module = await WebAssembly.compile(await wasmResponse.arrayBuffer());
  }

  setStatus("Instantiating...");
  // Some builds may still import symbols from the legacy "env" module.
  // Provide an (initially) empty env object so instantiation doesn't fail.
  const instance = await WebAssembly.instantiate(module, { env: {} });

  const e = instance.exports;
  if (!e.memory) {
    throw new Error("Missing export: memory");
  }
  if (typeof e.swisseph_alloc !== "function" || typeof e.swisseph_dealloc !== "function") {
    throw new Error("Missing exports: swisseph_alloc/swisseph_dealloc");
  }
  if (typeof e.swisseph_vfs_init !== "function") {
    throw new Error("Missing export: swisseph_vfs_init");
  }
  if (typeof e.swisseph_add_ephe_file !== "function") {
    throw new Error("Missing export: swisseph_add_ephe_file");
  }

  // Initialize in-wasm VFS.
  const initRc = e.swisseph_vfs_init();
  if (initRc !== 0) {
    throw new Error(`swisseph_vfs_init failed: ${readLastError(instance)}`);
  }
  appendOutput("[host] VFS initialized\n");

  // Load ephemeris assets and mount them into the in-wasm VFS store.
  const { mount, loadedFiles } = await loadEpheAssetsFromManifest();
  appendOutput(`[host] Loaded ${loadedFiles.length} ephemeris assets from web/assets\n`);

  for (const f of loadedFiles) {
    const nameBuf = writeUtf8ToWasm(instance, f.name);
    const dataPtr = e.swisseph_alloc(f.bytes.length);
    try {
      const mem = new Uint8Array(e.memory.buffer);
      mem.set(f.bytes, dataPtr);

      const addRc = e.swisseph_add_ephe_file(
        nameBuf.ptr,
        nameBuf.len,
        dataPtr,
        f.bytes.length,
      );
      if (addRc !== 0) {
        throw new Error(
          `swisseph_add_ephe_file(${f.name}) failed: ${readLastError(instance)}`,
        );
      }
    } finally {
      freeUtf8FromWasm(instance, nameBuf);
      e.swisseph_dealloc(dataPtr, f.bytes.length);
    }
  }

  appendOutput(`[host] Mounted ${loadedFiles.length} ephemeris files into VFS\n`);

  // Configure Swiss Ephemeris to use this logical path. The VFS backend may use
  // it to resolve requests.
  if (typeof e.swisseph_set_ephe_path_utf8 === "function") {
    const utf8 = writeUtf8ToWasm(instance, mount);
    try {
      const rc = e.swisseph_set_ephe_path_utf8(utf8.ptr, utf8.len);
      if (rc !== 0) {
        throw new Error(`swe_set_ephe_path failed: ${readLastError(instance)}`);
      }
      appendOutput(`[host] swe_set_ephe_path("${mount}") ok\n`);
    } finally {
      freeUtf8FromWasm(instance, utf8);
    }
  }

  setStatus("Ready");
  return { instance, mount, loadedFiles: new Set(loadedFiles.map((f) => f.name)) };
}

let wasmStatePromise = null;

async function getWasmState() {
  if (!wasmStatePromise) {
    wasmStatePromise = loadUnknownInstance();
  }
  return await wasmStatePromise;
}

function fmt(x) {
  if (!Number.isFinite(x)) return String(x);
  return x.toFixed(8);
}

async function calculatePlanets() {
  const { instance, loadedFiles } = await getWasmState();
  const e = instance.exports;

  if (typeof e.swisseph_julday_ut !== "function") {
    throw new Error("Missing export swisseph_julday_ut");
  }
  if (typeof e.swisseph_calc_ut !== "function") {
    throw new Error("Missing export swisseph_calc_ut");
  }

  const { year, month, day, hourUt } = parseUtcFromInputs();
  const jdUt = e.swisseph_julday_ut(year, month, day, hourUt);
  if (!Number.isFinite(jdUt)) {
    throw new Error(`julday failed: ${readLastError(instance)}`);
  }

  const iflag =
    (typeof e.swisseph_seflg_swieph === "function" ? e.swisseph_seflg_swieph() : 0) |
    (typeof e.swisseph_seflg_speed === "function" ? e.swisseph_seflg_speed() : 0);

  const rows = [];

  for (const p of PLANETS) {
    if (p.requires && loadedFiles && !loadedFiles.has(p.requires)) {
      appendOutput(
        `[host] Skipping ${p.name}: missing ephemeris file ${p.requires}\n`,
      );
      continue;
    }

    const ipl = p.getIpl(e);

    const out = allocF64(instance, 6);
    try {
      const rc = e.swisseph_calc_ut(jdUt, ipl, iflag, out.ptr);
      if (rc !== 0) {
        const msg = readLastError(instance) || "unknown error";
        appendOutput(`[host] calc_ut failed for ${p.name}: ${msg}\n`);
        rows.push({
          body: p.name,
          lon: "ERR",
          lat: "",
          dist: "",
          speedLon: "",
          speedLat: "",
          speedDist: "",
        });
        continue;
      }

      const mem = e.memory.buffer;
      const arr = new Float64Array(mem, out.ptr, 6);
      // Copy out of wasm memory before any further allocations.
      const [lon, lat, dist, speedLon, speedLat, speedDist] = Array.from(arr);

      rows.push({
        body: p.name,
        lon: fmt(lon),
        lat: fmt(lat),
        dist: fmt(dist),
        speedLon: fmt(speedLon),
        speedLat: fmt(speedLat),
        speedDist: fmt(speedDist),
      });
    } finally {
      freeF64(instance, out);
    }
  }

  renderResults(rows);
  appendOutput(`[host] Calculated ${rows.length} bodies at JD_UT=${jdUt}\n`);
}

function disableCalc(disabled) {
  const btn = document.getElementById("calc");
  btn.disabled = disabled;
}

document.addEventListener("DOMContentLoaded", () => {
  setDateDefaults();
  setStatus("Initializing...");

  getWasmState().catch((err) => {
    const msg =
      err && typeof err === "object" && "message" in err
        ? String(err.message)
        : String(err);
    appendOutput(`[host] Error during init: ${msg}\n`);
    if (err && typeof err === "object" && "stack" in err && err.stack) {
      appendOutput(`${err.stack}\n`);
    }
    setStatus("Error");
  });

  document.getElementById("calc").addEventListener("click", () => {
    disableCalc(true);
    setStatus("Calculating...");

    calculatePlanets()
      .then(() => {
        setStatus("Ready");
      })
      .catch((err) => {
        appendOutput(`[host] Error: ${err?.stack || err}\n`);
        setStatus("Error");
      })
      .finally(() => {
        disableCalc(false);
      });
  });
});

