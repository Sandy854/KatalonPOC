import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.apache.commons.io.FilenameUtils as FilenameUtils
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

//Initialisation des paramètres
nom_projet = "${nom_projet}"
numero_teledemarche = "${numero_teledemarche}"
date_validation = "${date_validation}"


//Récupérer le token
WebUI.callTestCase(findTestCase('TC001-Générer token'), [:], FailureHandling.STOP_ON_FAILURE)

'Déposer un dossier CU_OO6'
WS.sendRequestAndVerify(findTestObject('API Request/CU_006 DEPOSER DOSSIER AENV',[('nom_projet') : nom_projet , 
	('numero_teledemarche') : numero_teledemarche, ('date_validation') : date_validation]))

'Envoyer la requête CU_010 FICHIER RESTANT'
response = WS.sendRequestAndVerify(findTestObject('API Request/CU_010 FICHIER RESTANTS',[('numeroTeledemarche') : numero_teledemarche]), FailureHandling.CONTINUE_ON_FAILURE)

//Afficher dans le log le résultat JSON
System.out.println('REPONSE CU010 : ' + response.getResponseBodyContent())

//Convertir le résultat json en texte
'Récupérer le nombre de fichier à déposer'
def slurper = new groovy.json.JsonSlurper()

def jsonResult = slurper.parseText(response.getResponseBodyContent())

//Récupérer le nombre de fichier restant à déposer 
int nombreFichier = ((jsonResult.nombreFichiers) as int)

System.out.println('Nombre fichier à déposer : ' + nombreFichier)

'Evoyer la requête CU_008 DEPOSER FICHIER'
for (i = 0; i < nombreFichier; i++) {
    //Préparation de l'url du fichier à déposer
    String extensionFichier = FilenameUtils.getExtension(jsonResult.piecesJointes[i].nomFichier)

    String urlFile = 'C:\\MTE\\aaa.' + extensionFichier

    //Préparation du metadata JSON à déposer
    String metadata = ((((((((((((((((((((((('{' + '"identifiantFichier" : "') + jsonResult.piecesJointes[i].identifiantFichier) + 
    '",') + '"numeroTeledemarche" : "') + jsonResult.piecesJointes[i].numeroTeledemarche) + '",') + '"nomFichier" : "') + 
    jsonResult.piecesJointes[i].nomFichier) + '",') + '"nomOriginalFichier" : "') + jsonResult.piecesJointes[i].nomOriginalFichier) + 
    '",') + '"typeFichier" : "') + jsonResult.piecesJointes[i].typeFichier) + '",') + '"dateDepot" : "') + jsonResult.piecesJointes[
    i].dateDepot) + '",') + '"statutFichier" : "0",') + '"etatFichier" : "0",') + '"checkSum" : "') + jsonResult.piecesJointes[
    i].checkSum) + '"') + '}'

    System.out.println((i + '- METADATA : ') + metadata)

    'Envoyer la requête CU_008 dépôser fichier'
    response = WS.sendRequestAndVerify(findTestObject('API Request/CU_008 DEPOSER FICHIER AENV', [('metadata_body') : metadata
                , ('urlFile') : urlFile]), FailureHandling.CONTINUE_ON_FAILURE)

    System.out.println((i + '- REPONSE CU008 : ') + response.getResponseBodyContent())
}

