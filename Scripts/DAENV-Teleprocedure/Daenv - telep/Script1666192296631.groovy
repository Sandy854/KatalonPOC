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

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://pre-prod.psl.service-public.fr/pro_mademarche/DemandeAutorisationEnvironnementale/demarche')

WebUI.click(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 1/Bouton Accepter cookies'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Type de demande  Service-public.fr/input_Saisie complmentaire requise - affich_3e6a9c'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Type de demande  Service-public.fr/input_Numro dAIOT (le numro dAIOT doit comp_63a726'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Type de demande  Service-public.fr/input_Oui_chServInst'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Type de demande  Service-public.fr/input__infoReglo'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Type de demande  Service-public.fr/input__pieceConf'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Type de demande  Service-public.fr/input__planReglo'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Type de demande  Service-public.fr/span_Suivant'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Saisie complmentaire requise - affich_72546e'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Oui_chNPetitionnaire'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Personne morale_chPersonne'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Saisie complmentaire requise - affich_75517a'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Fminin_sexePP'))

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Jour (JJ)_jourPP'), 
    '12')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Mois (MM)_moisPP'), 
    '12')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Anne (AAAA)_anneePP'), 
    '2000')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Nom_nomPP'), 
    'Samandari')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Prnom_prenomPP'), 
    'Sandy')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_(Exemple 35510 CESSON SEVIGNE)_adress_68900f'), 
    '92400')

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/div_92400 - COURBEVOIE'))

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_(Exemple  72 rue Balzac)_adressePP.voie'), 
    '11')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Lieu-dit  boite postale  commune dlgu_58bd2d'), 
    'allée des tilleuls')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Tlphone fixe (Exemple 123456789)_bloc_418839'), 
    '122345678')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Tlphone portable (Exemple 623456789)__fd3d17'), 
    '612345678')

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Adresse lectronique (exemple  nomexem_61b427'), 
    'snd@yopmail.com')

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/input_Adresse lectronique (exemple  nomexem_00dd2b'))

'Fin de l\'étape 2 Pétitionnaire'
WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Ptitionnaire  Service-public.fr/span_Suivant'))

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Description du projet  Service-_a89986/input_Cration de la ZAC du Jas du Bouffon_P_f42f36'), 
    'V703-TNR-AENV-020')

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 3/uplodeFile_description_projet'), 'C:\\MTE\\aaa.pdf')

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 3/uplodeFile_presentation_technique'), 'C:\\MTE\\aaa.pdf')

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 3/uplodeFile_justificatif_foncier'), 'C:\\MTE\\aaa.pdf')

WebUI.delay(3)

'Fin de l\'étape 3 Description projet'
WebUI.click(findTestObject('Page_DAENV_optimiser/Page_DAENV -commun/Bouton Suivant'))

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Localisation  Service-public.fr/input_Saisie complmentaire requise - affich_f028f1'), 
    '31000')

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Localisation  Service-public.fr/div_Toulouse 31000'))

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Localisation  Service-public.fr/input_Saisie complmentaire requise - affich_248478'), 
    '11 allée des demois')

WebUI.delay(1)

WebUI.sendKeys(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Localisation  Service-public.fr/input_Saisie complmentaire requise - affich_248478'), 
    'elles')

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Localisation  Service-public.fr/div_11 Alle des Demoiselles'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Localisation  Service-public.fr/input_Votre projet est-il_typeprojet01'))

WebUI.click(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 4/Bouton_passer_a_l_information'))

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 4/uplodeFile_fichier_parcelles'), 'C:\\MTE\\aaa.csv')

WebUI.delay(3)

'Fin de l\'étape 4 Localisation'
WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Localisation  Service-public.fr/span_Suivant'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Activits  Service-public.fr/input_Oui_chRegularisation'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Activits  Service-public.fr/input_)_demande04'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Activits  Service-public.fr/span_Ajouter une catgorie'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Activits  Service-public.fr/select_SystmatiqueCas par cas'), 
    '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Activits  Service-public.fr/select_1 a) Installations classes mentionne_214f93'), 
    '1° a) Installations classées mentionnées à l’article L.515-28 du CE', false)

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Activits  Service-public.fr/button_Valider la ligne'))

'Fin de l\'étape 5 Activités'
WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Activits  Service-public.fr/span_Suivant'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  tude impactincidence  Service-p_a71781/input_Saisie complmentaire requise - affich_b44dc9'))

WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  tude impactincidence  Service-p_a71781/input_Saisie complmentaire requise - affich_38b3fd'))

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 6/uploadFile_etude_impact'), 'C:\\MTE\\aaa.pdf')

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 6/uploadFile_annexe'), 'C:\\MTE\\aaa.pdf')

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 6/uploadFile_resume_etude_impact'), 'C:\\MTE\\aaa.pdf')

WebUI.delay(3)

WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  tude impactincidence  Service-p_a71781/textarea_concat(Dcrivez l, , objet de votre_7a4a1d'), 
    'PRESENTATION TEST DU PROJET en 800 caractères maximum')

'Fin de l\'étape 6 Etude impact et incidence'
WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  tude impactincidence  Service-p_a71781/span_Suivant'))

'Fin de l\'étape 7 Pièces spécifiques'
WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Pices spcifiques  Service-public.fr/span_Suivant'))

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 8/uploadFile_plan_a_l_echelle'), 'C:\\MTE\\aaa.zip')

WebUI.uploadFile(findTestObject('Page_DAENV_optimiser/Page_DAENV - Etape 8/uploadFile_element_geographique'), 'C:\\MTE\\aaa.zip')

WebUI.delay(3)

'Fin de l\'étape 8 Plans'
WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Plans  Service-public.fr/button_Suivant'))

'Fin de l\'étape répapitulatif'
WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Rcapitulatif et envoi  Service-_ec2893/button_confirmer la demande'))

'CAPTCHA'
WebUI.setText(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Fin de dmarche  Service-public.fr/input_Veuillez renseigner les caractres aff_c59c5c'), 
    'tJswdh53')

'CAPTCHA'
WebUI.click(findTestObject('Object Repository/Page_DAENV/Page_DAENV  Fin de dmarche  Service-public.fr/span_Envoyer'))

