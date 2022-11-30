import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.By as By
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

nom_teleprocedure = "$nom_teleprocedure"

nom_service = "$service"

numero_aiot = "$numero_aiot"

WebUI.click(findTestObject('Page GUN/Generique - page/module GUN'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Page GUN/Bannette - page/Bouton Affecter', [('nom_teleprocedure') : nom_teleprocedure]))

WebUI.click(findTestObject('Page GUN/Affectation - page/bouton radio - IOTA'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Liste deroulante - Service'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Option - Service', [('service') : nom_service]))

WebUI.click(findTestObject('Page GUN/Affectation - page/Bouton Affecter'))

WebUI.click(findTestObject('Page GUN/Bannette - page/Bouton Affecter', [('nom_teleprocedure') : nom_teleprocedure]))

WebUI.click(findTestObject('Page GUN/Affectation - page/Liste deroulante - sousService'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Option - sousService'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Liste deroulante - equipe'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Option - equipe'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Bouton Affecter'))

WebUI.sendKeys(findTestObject('Page GUN/Affectation - page/Champ AIOT'), numero_aiot)

WebUI.click(findTestObject('Page GUN/Affectation - page/Bouton Rechercher'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Bouton Vérifier'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Bouton radio - AIOT present correspond'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Bouton Affecter Aiot'))

WebUI.click(findTestObject('Page GUN/Affectation - page/Bouton Ok - confirmer affectation'))

