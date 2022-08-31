import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('', FailureHandling.STOP_ON_FAILURE)

WebUI.maximizeWindow(FailureHandling.STOP_ON_FAILURE)

'Acceder au portail cerbere'
WebUI.navigateToUrl(GlobalVariable.ihm_gun_recette, FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementPresent(findTestObject('WebElement/Page connexion cerbere/Bouton cerbere bouchon'), 3)

'Se connecter à GUN'
WebUI.click(findTestObject('WebElement/Page connexion cerbere/Bouton cerbere bouchon'), FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementPresent(findTestObject('WebElement/Page connexion cerbere/Champ username'), 3)

WebUI.sendKeys(findTestObject('WebElement/Page connexion cerbere/Champ username'), GlobalVariable.utilisateur, FailureHandling.STOP_ON_FAILURE)

WebUI.sendKeys(findTestObject('WebElement/Page connexion cerbere/Champ password'), GlobalVariable.mot_de_passe, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('WebElement/Page connexion cerbere/Bouton Se connecter'), FailureHandling.STOP_ON_FAILURE)

'Vérifie la présence du lien d\'accès à la bannette GUN'
WebUI.waitForElementPresent(findTestObject('WebElement/Page Gun/Lien acces gun'), 3)

'Cliquer sur le lien bannette'
WebUI.click(findTestObject('WebElement/Page Gun/Lien acces gun'), FailureHandling.STOP_ON_FAILURE)

'Vérifie que je suis dans la page banette'
WebUI.waitForElementPresent(findTestObject('WebElement/Page Gun/Titre bannette'), 10)

'Rechercher le projet remonté'
TestObject colonneNomProjet = findTestObject('WebElement/Page Gun/Colonne nom projet')

List<WebElement> elements = WebUI.findWebElements(colonneNomProjet, 3)

isFound = false

for (WebElement element : elements) {
    if (element.getText().equals(String.valueOf("$nom_projet"))) {
        System.out

        isFound = true

        System.out.println('Nom du projet trouvé : ' + element.getText())

        break
    } else {
        isFound = false

        System.out.println((element.getText() + ' ne correspond pas à ') + "$nom_projet")
    }
}

'Vérifie si le projet est dans le tableau bannette'
WebUI.verifyEqual(isFound, true)

