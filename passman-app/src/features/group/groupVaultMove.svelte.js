import { get } from "svelte/store";
import { vaults, vaultData, currentVault } from "../vault/store.js";
import { moveGroupToVault, copyGroupToVault } from "./store.js";
import { showToast } from "../../stores/toast.js";

export function createGroupVaultMove(vaultPath, getGroupName) {
  let moveToVaultTarget = $state(null);
  let moveToVaultGroup = $state("");
  let moveToVaultAction = $state("move");

  function reset() {
    moveToVaultTarget = null;
    moveToVaultGroup = "";
    moveToVaultAction = "move";
  }

  async function handleVaultAction(detail, action, closeContextMenu) {
    const { sourceId, targetPath } = detail;
    const target = get(vaults).find((v) => v.path === targetPath);
    const targetGroups = (get(vaultData)[targetPath]?.groups || []).map(
      (g) => g.id,
    );
    if (target && targetGroups.includes(sourceId)) {
      moveToVaultGroup = sourceId;
      moveToVaultTarget = target;
      moveToVaultAction = action;
      closeContextMenu();
    } else if (target) {
      try {
        const fn = action === "copy" ? copyGroupToVault : moveGroupToVault;
        await fn(sourceId, targetPath, sourceId);
        currentVault.set(target);
        showToast(
          `${action === "copy" ? "Copied" : "Moved"} "${getGroupName(sourceId)}" to ${target.name}`,
        );
      } catch (e) {
        console.error(e);
        alert(`${action === "copy" ? "Copy" : "Move"} failed: ${e}`);
      }
      closeContextMenu();
    }
  }

  async function handleResolve(targetId, closeContextMenu) {
    if (moveToVaultTarget && moveToVaultGroup) {
      try {
        const fn =
          moveToVaultAction === "copy" ? copyGroupToVault : moveGroupToVault;
        await fn(moveToVaultGroup, moveToVaultTarget.path, targetId);
        currentVault.set(moveToVaultTarget);
        const verb = moveToVaultAction === "copy" ? "Copied" : "Moved";
        if (targetId === moveToVaultGroup) {
          showToast(
            `${verb} "${getGroupName(moveToVaultGroup)}" into ${moveToVaultTarget.name}`,
          );
        } else {
          showToast(
            `${verb} "${getGroupName(moveToVaultGroup)}" to ${moveToVaultTarget.name} as "${getGroupName(targetId)}"`,
          );
        }
      } catch (e) {
        console.error(e);
        alert(`${moveToVaultAction === "copy" ? "Copy" : "Move"} failed: ${e}`);
      }
    }
    reset();
    closeContextMenu();
  }

  return {
    get moveToVaultTarget() { return moveToVaultTarget; },
    get moveToVaultGroup() { return moveToVaultGroup; },
    get moveToVaultAction() { return moveToVaultAction; },
    handleVaultAction,
    handleResolve,
    reset,
  };
}
