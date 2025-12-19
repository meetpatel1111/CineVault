from playwright.sync_api import sync_playwright

def verify_filter_ui():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Navigate to app
        try:
            page.goto("http://localhost:1420")
            page.wait_for_load_state("networkidle")

            # Click Filter button (search icon)
            # The button has 'Filter' text or search icon
            page.get_by_role("button", name="Filter").click()

            # Wait for panel to appear
            # Look for "Filter Media" heading
            page.wait_for_selector("text=Filter Media")

            # Interact with inputs
            page.fill("input[placeholder='Min']", "2000")
            page.fill("input[placeholder='Max']", "2023")

            # Check a resolution box
            # Use specific locator for the checkbox within the label
            page.locator("label").filter(has_text="1080p / Full HD").locator("input[type=checkbox]").check()

            # Check a media type box
            page.locator("label").filter(has_text="Movies").locator("input[type=checkbox]").check()

            # Take screenshot
            page.screenshot(path="/home/jules/verification/filter_ui.png")
            print("Screenshot taken")

        except Exception as e:
            print(f"Error: {e}")
            page.screenshot(path="/home/jules/verification/error.png")
        finally:
            browser.close()

if __name__ == "__main__":
    verify_filter_ui()
