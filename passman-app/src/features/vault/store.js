import { get, writable, derived } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { SAVE_LISTENER_TIMEOUT_MS } from "../../lib/constants.js";
import { showToast } from "../../stores/toast.js";
import { deleteVaultStore } from "../../stores/selection.js";

export const vaults = writable([]);
export const currentVault = writable(null);
export const vaultData = writable({});


export const isUnlocked = derived(
  [currentVault, vaultData],
  ([$currentVault, $vaultData]) => {
    return $currentVault ? $vaultData[$currentVault.path]?.unlocked === true : false;
  }
);

export const groups = derived(
  [currentVault, vaultData],
  ([$currentVault, $vaultData]) => {
    return $currentVault ? $vaultData[$currentVault.path]?.groups || [] : [];
  }
);

export const entries = derived(
  [currentVault, vaultData],
  ([$currentVault, $vaultData]) => {
    return $currentVault ? $vaultData[$currentVault.path]?.entries || [] : [];
  }
);

let saveUnlisten = null;

export async function initSaveListener() {
  if (saveUnlisten) return saveUnlisten;
  const listenPromise = listen("save-status", (event) => {
    if (event.payload === "saved") {
      showToast("Saved");
    } else if (event.payload === "error") {
      showToast("Save failed");
    }
  });
  const timeoutPromise = new Promise((_, reject) =>
    setTimeout(() => reject(new Error("save listener timeout")), SAVE_LISTENER_TIMEOUT_MS)
  );
  saveUnlisten = await Promise.race([listenPromise, timeoutPromise]);
  return saveUnlisten;
}

export function updateVaultData(path, data) {
  vaultData.update((vd) => {
    const existing = vd[path] || {};
    return { ...vd, [path]: { ...existing, ...data } };
  });
}

function clearVaultData(path) {
  vaultData.update((vd) => {
    const { [path]: _, ...rest } = vd;
    return rest;
  });
}

function setVaultData(path, vault) {
  updateVaultData(path, {
    unlocked: true,
    groups: vault.groups || [],
    tags: vault.tags || [],
    entries: vault.entries || [],
    trash: vault.trash || { groups: [], entries: [] },
  });
}

export async function createVault(id, name, path, password) {
  const vault = await invoke("create_vault", { id, name, path, password });
  vaults.update((list) => [...list, vault]);
  currentVault.set(vault);
  setVaultData(path, vault);
  return vault;
}

export async function openVault(path, password) {
  const vault = await invoke("open_vault", { path, password });
  const existing = get(currentVault);
  currentVault.set({
    ...existing,
    ...vault,
    name: vault.name || existing?.name,
    path: vault.path || existing?.path,
  });
  setVaultData(path, vault);
  return vault;
}

export async function registerAndOpenVault(id, path, password) {
  const vault = await invoke("register_and_open_vault", { id, path, password });
  vaults.update((list) => [...list, { id, name: vault.name, path }]);
  currentVault.set({ ...vault, id });
  setVaultData(path, vault);
  return vault;
}

export async function closeVault(path) {
  await invoke("close_vault", { path });
  if (get(currentVault)?.path === path) {
    currentVault.set(null);
  }
  clearVaultData(path);
  deleteVaultStore(path);
}

export const lockVaultByPath = closeVault;

export async function lockVault() {
  const vault = get(currentVault);
  if (!vault) return;
  await lockVaultByPath(vault.path);
}

export async function unlockVault(password) {
  const vault = get(currentVault);
  if (!vault) return;
  const opened = await invoke("open_vault", { path: vault.path, password });
  currentVault.set({
    ...vault,
    ...opened,
    name: opened.name || vault.name,
    path: opened.path || vault.path,
  });
  setVaultData(vault.path, opened);
}

export async function deleteVault(id, path) {
  await invoke("delete_vault", { id, path });
  vaults.update((list) => list.filter((v) => v.id !== id));
  clearVaultData(path);
  if (get(currentVault)?.path === path) {
    currentVault.set(null);
  }
}

export async function renameVault(id, name) {
  const updated = await invoke("rename_vault", { id, name });
  vaults.update((list) =>
    list.map((v) => (v.id === id ? { ...v, name } : v)),
  );
  currentVault.update((v) => {
    if (v && v.id === id) {
      return { ...v, name };
    }
    return v;
  });
  return updated;
}

export async function reorderVaults(orderedIds) {
  const list = await invoke("reorder_vaults", { ids: orderedIds });
  vaults.set(list || []);
  return list;
}

export async function convertButtercupVault(bcupPath, password, outputPath) {
  const id = crypto.randomUUID();
  const vault = await invoke("convert_buttercup_vault", { bcupPath, password, outputPath, id });
  vaults.update((list) => [...list, { id, name: vault.name, path: outputPath }]);
  currentVault.set({ ...vault, id });
  setVaultData(outputPath, vault);
  return vault;
}

export async function convertKeepassVault(kdbxPath, password, outputPath) {
  const id = crypto.randomUUID();
  const vault = await invoke("convert_keepass_vault", { kdbxPath, password, outputPath, id });
  vaults.update((list) => [...list, { id, name: vault.name, path: outputPath }]);
  currentVault.set({ ...vault, id });
  setVaultData(outputPath, vault);
  return vault;
}
