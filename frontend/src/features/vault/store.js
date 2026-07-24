import { get, writable, derived } from "svelte/store";
import { Events } from "@wailsio/runtime";
import { SAVE_LISTENER_TIMEOUT_MS } from "../../lib/constants.js";
import { showToast } from "../../stores/toast.js";
import { deleteVaultStore } from "../../stores/selection.js";
import * as vaultService from "../../../bindings/github.com/eqto/passman/internal/app/vaultservice.js";

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
  const listenPromise = Events.On("save-status", (event) => {
    if (event.data === "saved") {
      showToast("Saved");
    } else if (event.data === "error") {
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

export async function createVault(id, name, path, password, securityLevel) {
  const vault = await vaultService.CreateVault(id, name, path, password, securityLevel ?? null);
  vaults.update((list) => [...list, vault]);
  currentVault.set(vault);
  setVaultData(path, vault);
  return vault;
}

export async function openVault(path, password) {
  const vault = await vaultService.OpenVault(path, password);
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
  const vault = await vaultService.RegisterAndOpenVault(id, path, password);
  const name = vault.name || path.split("/").pop().replace(/\.pmv$/i, "");
  vaults.update((list) => [...list, { id, name, path }]);
  currentVault.set({ ...vault, id, name });
  setVaultData(path, vault);
  return vault;
}

export async function closeVault(path) {
  await vaultService.CloseVault(path);
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
  const opened = await vaultService.OpenVault(vault.path, password);
  currentVault.set({
    ...vault,
    ...opened,
    name: opened.name || vault.name,
    path: opened.path || vault.path,
  });
  setVaultData(vault.path, opened);
}

export async function deleteVault(id, path) {
  await vaultService.DeleteVault(id, path);
  vaults.update((list) => list.filter((v) => v.id !== id));
  clearVaultData(path);
  if (get(currentVault)?.path === path) {
    currentVault.set(null);
  }
}

export async function renameVault(id, name) {
  const updated = await vaultService.RenameVault(id, name);
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
  const list = await vaultService.ReorderVaults(orderedIds);
  vaults.set(list || []);
  return list;
}

export async function convertButtercupVault(bcupPath, password, outputPath, securityLevel) {
  const id = crypto.randomUUID();
  const vault = await vaultService.ConvertButtercupVault(bcupPath, password, outputPath, id, securityLevel ?? null);
  vaults.update((list) => [...list, { id, name: vault.name, path: outputPath }]);
  currentVault.set({ ...vault, id });
  setVaultData(outputPath, vault);
  return vault;
}

export async function convertKeepassVault(kdbxPath, password, outputPath, securityLevel) {
  const id = crypto.randomUUID();
  const vault = await vaultService.ConvertKeepassVault(kdbxPath, password, outputPath, id, securityLevel ?? null);
  vaults.update((list) => [...list, { id, name: vault.name, path: outputPath }]);
  currentVault.set({ ...vault, id });
  setVaultData(outputPath, vault);
  return vault;
}

export async function changeSecurityLevel(path, password, newLevel) {
  await vaultService.ChangeSecurityLevel(path, password, newLevel);
}
