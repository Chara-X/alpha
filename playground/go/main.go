package main

import (
	"fmt"
	"log"
	"regexp"

	"github.com/playwright-community/playwright-go"
)

func main() {
	var pw, _ = playwright.Run()
	defer pw.Stop()
	var browser, _ = pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	defer browser.Close()
	var page, _ = browser.NewPage(playwright.BrowserNewPageOptions{
		Permissions: []string{},
		StorageState: &playwright.OptionalStorageState{
			Cookies: []playwright.OptionalCookie{},
		},
		ExtraHttpHeaders: map[string]string{},
		Geolocation:      &playwright.Geolocation{},
		Offline:          playwright.Bool(false),
	})
	defer page.Close()
	page.Goto("https://www.w3schools.com/html/html_iframe.asp")
	// var getStarted = page.Locator(".getStarted_Sjon")
	// fmt.Println(getStarted.Evaluate("(x,y) => y.innerText",getStarted))
	var ctx = page.Context()
	ctx.StorageState()
	page.GetByRole("", playwright.PageGetByRoleOptions{})
	page.GetByText("")
	var asserts = playwright.NewPlaywrightAssertions()
	asserts.Locator(page.Locator("")).ToHaveText("")
	ctx.Request().StorageState("state.json")
}
func ExamplePlaywright() {
	// Start Playwright
	pw, err := playwright.Run()
	if err != nil {
		log.Fatalf("could not start Playwright: %v", err)
	}
	defer pw.Stop()
	// Launch a browser
	browser, err := pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	if err != nil {
		log.Fatalf("could not launch browser: %v", err)
	}
	defer browser.Close()
	// Create a new page
	page, err := browser.NewPage()
	if err != nil {
		log.Fatalf("could not create page: %v", err)
	}
	// Test 1: Check if the title contains "Playwright"
	_, err = page.Goto("https://playwright.dev/")
	if err != nil {
		log.Fatalf("could not visit page: %v", err)
	}
	title, err := page.Title()
	if err != nil {
		log.Fatalf("could not get title: %v", err)
	}
	// Use regular expression to check the title
	match, _ := regexp.MatchString("Playwright", title)
	if match {
		fmt.Println("Test 1 passed: Title contains 'Playwright'")
	} else {
		fmt.Println("Test 1 failed: Title does not contain 'Playwright'")
	}
	// Test 2: Click the 'Get started' link and check for heading "Installation"
	_, err = page.Goto("https://playwright.dev/")
	if err != nil {
		log.Fatalf("could not visit page: %v", err)
	}
	// Click the 'Get started' link
	// err = page.Locator("text=Get started").Click()
	x := page.GetByRole("link", playwright.PageGetByRoleOptions{
		Name: "Get started",
	})
	fmt.Println(x.InnerText())
	x.Click()
	if err != nil {
		log.Fatalf("could not click on 'Get started' link: %v", err)
	}
	// Wait for the 'Installation' heading to be visible
	// heading, err := page.Locator("text=Installation").IsVisible()
	x = page.GetByRole("heading", playwright.PageGetByRoleOptions{
		Name: "Installation",
	})
	fmt.Println(x.InnerText())
	if err != nil {
		log.Fatalf("could not check visibility of 'Installation' heading: %v", err)
	}
	var visible bool
	playwright.NewPlaywrightAssertions().Locator(x).ToBeVisible(playwright.LocatorAssertionsToBeVisibleOptions{
		Visible: &visible,
	})
	fmt.Println(x.AriaSnapshot())
	if visible {
		fmt.Println("Test 2 passed: 'Installation' heading is visible")
	} else {
		fmt.Println("Test 2 failed: 'Installation' heading is not visible")
	}
}

/*
curl -i -X POST "https://swr:6000/swr/v1/tenants/admin/images/upload?name=image/birensupa-pytorch-llama2_70b&version=v7.24.40.06" -F 'file=@birensupa-pytorch-llama2_70b-v7.24.40.06.tar'

curl -i -k -X POST https://10.166.209.110:443/opapi/wsm/v1/apts/dataset \
  -H "Content-Type: application/zip" \
  -F "file=@/home/pict/ntq/wudao.zip"

curl -i -k -X POST https://10.166.209.110:443/opapi/wsm/v1/apts/dataset/bind -d '{"datasetName":"wudao22","pvc":"llama70b","namespace":"test70b","datasetFileName":"wudao.zip"}'
*/
