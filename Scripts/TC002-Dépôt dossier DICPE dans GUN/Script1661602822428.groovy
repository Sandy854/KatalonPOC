import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

//Génération du token
WebUI.callTestCase(findTestCase('TC001-Générer token'), [:], FailureHandling.STOP_ON_FAILURE)

'Envoi du requête CU006 DEPOSER DOSSIER'
response = WS.sendRequest(findTestObject('API Request/CU_006 DEPOSER DOSSIER', [('nom_projet') : "$nom_projet", ('numero_teledemarche') : "$numero_teledemarche"
            , ('date_validation') : "$date_validation"]))

//Afficher dans le log le résultat JSON
System.out.println('REPONSE CU006 : ' + response.getResponseBodyContent())

//Récupérer les attributs JSON nécessaire au dépot fichier
def slurper = new groovy.json.JsonSlurper()

def jsonResult = slurper.parseText(response.getResponseBodyContent())

'Envoi du requête CU008 DEPOSER FICHIER'
String metadata = ((((((((((((((((((((((('{' + '"identifiantFichier" : "') + jsonResult.orientationDemande.fichierAccuseReception.identifiantFichier) + 
'",') + '"numeroTeledemarche" : "') + jsonResult.numeroTeledemarche) + '",') + '"nomFichier" : "') + jsonResult.orientationDemande.fichierAccuseReception.nomFichier) + 
'",') + '"nomOriginalFichier" : "') + jsonResult.orientationDemande.fichierAccuseReception.nomOriginalFichier) + '",') + 
'"typeFichier" : "') + jsonResult.orientationDemande.fichierAccuseReception.typeFichier) + '",') + '"dateDepot" : "') + 
jsonResult.orientationDemande.fichierAccuseReception.dateDepot) + '",') + '"statutFichier" : "0",') + '"etatFichier" : "0",') + 
'"checkSum" : "') + jsonResult.orientationDemande.fichierAccuseReception.checkSum) + '"') + '}'

response = WS.sendRequest(findTestObject('API Request/CU_008 DEPOSER FICHIER', [('metadata_body') : metadata]))

System.out.println('REPONSE CU008 : ' + response.getResponseBodyContent())

'Véfication dans GUN si le dossier est bien remonté'
WebUI.callTestCase(findTestCase('TC003-Recherche du dossier déposé dans GUN'), [('nom_projet') : jsonResult.orientationDemande.projetNom], 
    FailureHandling.STOP_ON_FAILURE)

