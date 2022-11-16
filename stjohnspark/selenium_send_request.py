from selenium.webdriver.common.by import By
from selenium import webdriver
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.chrome.service import Service as ChromeService
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.chrome.options import Options
import time


def get_driver():
    options = Options()
    options.add_argument("--user-data-dir=chrome-data")
    options.add_experimental_option("excludeSwitches", ["enable-automation"])
    options.add_experimental_option('useAutomationExtension', False)
    driver = webdriver.Chrome(service=ChromeService(ChromeDriverManager(path=".").install()),
    options=options)
    driver.get("https://web.whatsapp.com/")
    return driver


def send_message(driver, target, message_txt):
    try:
        wait = WebDriverWait(driver, 600)
        target = '"{}"'.format(target)
        x_arg = "//span[contains(@title," + target + ")]"
        group_title = wait.until(EC.presence_of_element_located((By.XPATH, x_arg)))
        print(group_title)
        print("Wait for few seconds")
        group_title.click()
        xpath_message_box = driver.find_element(
            "xpath",
            '//*[@id="main"]/footer/div[1]/div/span[2]/div/div[2]/div[1]/div/div[1]'
        )
        if xpath_message_box is not None:
            message = xpath_message_box
        else:
            print('/??????????????????')
            raise Exception
        message.send_keys(message_txt)
        sendbutton = driver.find_element(
            "xpath",
            '//*[@id="main"]/footer/div[1]/div/span[2]/div/div[2]/div[2]/button'
        )
        sendbutton.click()
        print("Message Sent")
        time.sleep(10)
    except Exception as e:
        print(f"Unable to Send Message to {target} Error:{e}")
        while(True):
            pass

