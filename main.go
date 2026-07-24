package main

import (
	"embed"
	"log"

	"github.com/eqto/passman/internal/app"
	"github.com/eqto/passman/internal/state"
	"github.com/eqto/passman/pkg/vault"

	"github.com/wailsapp/wails/v3/pkg/application"
	"github.com/wailsapp/wails/v3/pkg/events"
)

//go:embed all:frontend/dist
var assets embed.FS

//go:embed build/appicon.png
var trayIcon []byte

func main() {
	saveCh := make(chan state.SaveJob, 64)

	go func() {
		for job := range saveCh {
			if err := vault.SaveVaultFileWithKey(job.Vault, job.Key); err != nil {
				log.Printf("save error: %v", err)
			}
		}
	}()

	appState := state.NewAppState(saveCh)

	vaultSvc := app.NewVaultService(appState)
	groupSvc := app.NewGroupService(appState)
	entrySvc := app.NewEntryService(appState)
	passwordSvc := app.NewPasswordService()

	wailsApp := application.New(application.Options{
		Name:        "Passman",
		Description: "A secure password manager",
		Services: []application.Service{
			application.NewService(vaultSvc),
			application.NewService(groupSvc),
			application.NewService(entrySvc),
			application.NewService(passwordSvc),
		},
		Assets: application.AssetOptions{
			Handler: application.AssetFileServerFS(assets),
		},
		Mac: application.MacOptions{
			ApplicationShouldTerminateAfterLastWindowClosed: false,
		},
	})

	mainWindow := wailsApp.Window.NewWithOptions(application.WebviewWindowOptions{
		Title:            "Passman",
		Width:            1000,
		Height:           700,
		MinWidth:         800,
		MinHeight:        600,
		BackgroundColour: application.NewRGB(27, 38, 54),
		URL:              "/",
	})

	trayMenu := application.NewMenu()
	trayMenu.Add("Show Passman").OnClick(func(ctx *application.Context) {
		mainWindow.Show().Focus()
	})
	trayMenu.AddSeparator()
	trayMenu.Add("Quit").OnClick(func(ctx *application.Context) {
		wailsApp.Quit()
	})

	tray := wailsApp.SystemTray.New()
	tray.SetIcon(trayIcon)
	tray.SetTooltip("Passman — click to show/hide")
	tray.SetMenu(trayMenu)
	tray.OnClick(func() {
		if mainWindow.IsVisible() {
			mainWindow.Hide()
		} else {
			mainWindow.Show().Focus()
		}
	})

	mainWindow.RegisterHook(events.Common.WindowClosing, func(event *application.WindowEvent) {
		event.Cancel()
		mainWindow.Hide()
	})

	if err := wailsApp.Run(); err != nil {
		log.Fatal(err)
	}
}
