package main

import (
	"fmt"
	"log"
	"regexp"

	"github.com/playwright-community/playwright-go"
)

func main() {
	ExamplePlaywright()
}
func ExampleHackNews() {
	{
		pw, err := playwright.Run()
		if err != nil {
			log.Fatalf("could not start playwright: %v", err)
		}
		browser, err := pw.Chromium.Launch()
		if err != nil {
			log.Fatalf("could not launch browser: %v", err)
		}
		page, err := browser.NewPage()
		if err != nil {
			log.Fatalf("could not create page: %v", err)
		}
		if _, err = page.Goto("https://news.ycombinator.com"); err != nil {
			log.Fatalf("could not goto: %v", err)
		}
		entries, err := page.Locator(".athing").All()
		if err != nil {
			log.Fatalf("could not get entries: %v", err)
		}
		for i, entry := range entries {
			title, err := entry.Locator("td.title > span > a").TextContent()
			if err != nil {
				log.Fatalf("could not get text content: %v", err)
			}
			fmt.Printf("%d: %s\n", i+1, title)
		}
		if err = browser.Close(); err != nil {
			log.Fatalf("could not close browser: %v", err)
		}
		if err = pw.Stop(); err != nil {
			log.Fatalf("could not stop Playwright: %v", err)
		}
	}
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
	x.Click()
	x.Highlight()
	x.SetChecked(true)
	x.Uncheck()
	if visible {
		fmt.Println("Test 2 passed: 'Installation' heading is visible")
	} else {
		fmt.Println("Test 2 failed: 'Installation' heading is not visible")
	}
}
func ExamplePakWheels() {
	pw, _ := playwright.Run()
	defer pw.Stop()
	browser, _ := pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	defer browser.Close()
	page, _ := browser.NewPage()
	page.Goto("https://www.pakwheels.com/")
	searchBox := page.Locator("input[name='home-query']")
	btn := page.Locator("button[id='home-search-btn']")
	searchBox.Fill("Toyota Corolla")
	btn.Click()
	if err := page.WaitForLoadState(); err != nil {
		panic(err)
	}
	classifiedListings := page.Locator("li.classified-listing")
	count, _ := classifiedListings.Count()
	fmt.Println(count)
	for i := 0; i < count; i++ {
		listing := classifiedListings.Nth(i)
		fmt.Println(listing.InnerText())
		_, err := listing.Screenshot(playwright.LocatorScreenshotOptions{
			Path: playwright.String(fmt.Sprintf("capture/listing-%d.png", i+1)),
		})
		if err != nil {
			panic(err)
		}
	}
}
