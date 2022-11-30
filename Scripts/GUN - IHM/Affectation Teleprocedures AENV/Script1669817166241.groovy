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
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('GUN - IHM/Login Gun'), [('utilisateur') : '6_adminNational@gun-rec.fr', ('mot_de_passe') : '6_adminNational@gun-rec.fr'], 
    FailureHandling.STOP_ON_FAILURE)

for (i = 1; i <= 20; i++) {
    numero_test = i

    if (i < 10) {
        numero_test = ('0' + i)
    }
    
    teleprocedure = ('V703-TNR-AENV-0' + numero_test)

    WebUI.callTestCase(findTestCase('GUN - IHM/Affectation Teleprocedure AENV'), [('nom_teleprocedure') : teleprocedure, ('numero_aiot') : '0100008327'
            , ('service') : 'DDT 31', ('sous_service') : 'SEEF', ('equipe') : 'Unité des procédures environnementales'], 
        FailureHandling.STOP_ON_FAILURE)
}

