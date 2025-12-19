from playwright.sync_api import sync_playwright

def verify_metadata_editor():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Navigate to app
        try:
            page.goto("http://localhost:1420")
            page.wait_for_load_state("networkidle")

            # Find a media card and open its menu
            page.wait_for_selector(".media-card")

            # Hover over first card
            card = page.locator(".media-card").first
            card.hover()

            # Force click the menu button to avoid overlay issues (e.g., progress bar)
            menu_btn = card.locator(".media-card__menu-button")
            menu_btn.click(force=True)

            # Click "Edit Metadata" - also force if needed, but usually dropdowns are on top
            page.click("text=Edit Metadata", force=True)

            # Verify Modal appears
            page.wait_for_selector("text=Edit Metadata")

            # Fill out form
            page.fill("input[placeholder='Movie or Episode Title']", "New Verified Title")
            page.fill("input[placeholder='YYYY']", "2024")
            page.fill("textarea", "Verified description")

            # Take screenshot of the editor
            page.screenshot(path="/home/jules/verification/metadata_editor.png")
            print("Editor Screenshot taken")

        except Exception as e:
            print(f"Error: {e}")
            page.screenshot(path="/home/jules/verification/error_editor.png")
        finally:
            browser.close()

if __name__ == "__main__":
    verify_metadata_editor()
