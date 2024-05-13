"use client"

import { useCallback, useEffect, useState } from "react"
import logo from "@/assets/logo.png"
import { Globe, Mic, Sailboat } from "lucide-react"
import { WindowTitlebar } from "tauri-controls"

import { Dialog, DialogTrigger } from "@/components/ui/dialog"
import {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarItem,
  MenubarLabel,
  MenubarMenu,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from "@/components/ui/menubar"
import { MenuModeToggle } from "@/components/navigation/menu-mode-toggle"

export function Navigation() {
  const closeWindow = useCallback(async () => {
    const { appWindow } = await import("@tauri-apps/plugin-window")
    appWindow.close()
  }, [])

  return (
    <WindowTitlebar
    // controlsOrder="platform"
    // windowControlsProps={{ platform: "macos", className: "" }}
    >
      <Menubar className="rounded-none border-b border-none pl-2 lg:pl-3">
        <MenubarMenu>
          <div className="inline-flex h-fit w-fit items-center text-cyan-500">
            <Sailboat className="h-5 w-5" />
          </div>
        </MenubarMenu>

        <MenubarMenu>
          <MenubarTrigger className="font-bold">Play</MenubarTrigger>
          <Dialog modal={false}>
            <MenubarContent>
              <DialogTrigger asChild>
                <MenubarItem>About App</MenubarItem>
              </DialogTrigger>
              <MenubarSeparator />
              <MenubarItem>
                New League <MenubarShortcut>⌘N</MenubarShortcut>
              </MenubarItem>
              <MenubarItem>
                Open League<MenubarShortcut>⌘O</MenubarShortcut>
              </MenubarItem>
              <MenubarItem>
                Load League<MenubarShortcut>⇧⌘L</MenubarShortcut>
              </MenubarItem>
              <MenubarSeparator />
              <MenubarItem onClick={closeWindow}>
                Manage Leages <MenubarShortcut>⌘Q</MenubarShortcut>
              </MenubarItem>
            </MenubarContent>
          </Dialog>
        </MenubarMenu>
        <MenuModeToggle />
      </Menubar>
    </WindowTitlebar>
  )
}
