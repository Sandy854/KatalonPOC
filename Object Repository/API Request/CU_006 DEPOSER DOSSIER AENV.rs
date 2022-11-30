<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CU_006 DEPOSER DOSSIER AENV</name>
   <tag></tag>
   <elementGuidId>e01d3901-5a60-46b2-9983-1f59c63ec737</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;code\&quot; : \&quot;DILA\&quot;,\n  \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n  \&quot;metaDataDossier\&quot; : {\n    \&quot;numeroDossier\&quot; : \&quot;\&quot;,\n    \&quot;numeroDemarche\&quot; : \&quot;AENV\&quot;,\n    \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n    \&quot;dateValidation\&quot; : \&quot;${date_validation}\&quot;,\n    \&quot;nombreFichiers\&quot; : 0,\n    \&quot;identificationDemande\&quot; : 1,\n    \&quot;orientationDemande\&quot; : {\n      \&quot;codeAIOTInconnu\&quot; : true,\n      \&quot;serviceInstructeur\&quot; : \&quot;0\&quot;,\n      \&quot;certificatProjetNumero\&quot; : \&quot;\&quot;,\n      \&quot;engagementFichiersDeposes\&quot; : true,\n      \&quot;engagementPiecesNonconfidentielles\&quot; : true,\n      \&quot;engagementPlans\&quot; : true,\n      \&quot;petitionnaires\&quot; : [ {\n        \&quot;numeroPetitionnaire\&quot; : \&quot;1\&quot;,\n        \&quot;typePetitionnaire\&quot; : \&quot;2\&quot;,\n        \&quot;siret\&quot; : \&quot;70204275500109\&quot;,\n        \&quot;petitionnaireAdresse\&quot; : {\n          \&quot;adresse1\&quot; : \&quot;17 PL DES REFLETS\&quot;,\n          \&quot;adresse2\&quot; : \&quot; -- \&quot;,\n          \&quot;adresse3\&quot; : \&quot;IMMEUBLE CB 16\&quot;,\n          \&quot;commune\&quot; : {\n            \&quot;codePostal\&quot; : \&quot;92400\&quot;,\n            \&quot;commune\&quot; : \&quot;COURBEVOIE\&quot;,\n            \&quot;pays\&quot; : \&quot;FR\&quot;,\n            \&quot;codeInsee\&quot; : \&quot;92026\&quot;\n          }\n        },\n        \&quot;personnePhysique\&quot; : {\n          \&quot;civilite\&quot; : \&quot;1\&quot;,\n          \&quot;nom\&quot; : \&quot;GERIN\&quot;,\n          \&quot;prenom\&quot; : \&quot;LAURENT\&quot;,\n          \&quot;dateDeNaissance\&quot; : \&quot;2000-02-02\&quot;\n        },\n        \&quot;personnePhysiqueContact\&quot; : {\n          \&quot;telephoneFixe\&quot; : \&quot;0123456787\&quot;,\n          \&quot;telephoneMobile\&quot; : \&quot;0654323456\&quot;,\n          \&quot;email\&quot; : \&quot;snd@yopmail.com\&quot;\n        },\n        \&quot;personnePhysiqueAnonyme\&quot; : false\n      } ],\n      \&quot;emailEchangeAvecAdministration\&quot; : \&quot;snd@yopmail.com\&quot;,\n      \&quot;etesVousPetitionnaire\&quot; : true,\n      \&quot;demandeAInstallationIOTA\&quot; : true,\n      \&quot;demandeAInstallationICPE\&quot; : false,\n      \&quot;demandeEInstallationICPEBasculeAEnv\&quot; : false,\n      \&quot;demandeASuppletive\&quot; : false,\n      \&quot;demandeConcerneEnrICPE\&quot; : false,\n      \&quot;demandeConcerneDICPE\&quot; : false,\n      \&quot;demandeConcerneDIOTA\&quot; : false,\n      \&quot;demandeConcerneAIOTDerogationEspecesProteges\&quot; : false,\n      \&quot;demandeConcerneAIOTAutorisationDefrichement\&quot; : false,\n      \&quot;demandeConcerneAIOTNatura2000\&quot; : false,\n      \&quot;demandeConcerneAIOTGazEffetSerre\&quot; : false,\n      \&quot;demandeModificationAspectReserveNaturelle\&quot; : false,\n      \&quot;demandeModificationAspectSiteClasse\&quot; : false,\n      \&quot;demandeDossierAgrementOGM\&quot; : false,\n      \&quot;demandeDossierAgrementDechets\&quot; : false,\n      \&quot;demandeInstallationElectriciteMecaniqueVent\&quot; : false,\n      \&quot;demandeInstallationElectriciteRequerantAutorisation\&quot; : false,\n      \&quot;demandeInfraTerrestreLineaireTransport\&quot; : false,\n      \&quot;demandeDerogSDAGE\&quot; : false,\n      \&quot;projetNom\&quot; : \&quot;${nom_projet}\&quot;,\n      \&quot;fichierDescriptionProjet\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierDescriptionProjet.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;000\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;fichierNotePresentationNonTechnique\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierNotePresentationNonTechnique.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;001\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;fichierSyntheseMesuresEnvisagees\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierSyntheseMesuresEnvisagees.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;002\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;memeAdresseAIOTPetitionnaire\&quot; : false,\n      \&quot;adressePrincipaleAIOT\&quot; : {\n        \&quot;adresse1\&quot; : \&quot;11 Rue Reguelongue\&quot;,\n        \&quot;adresse2\&quot; : \&quot; -- \&quot;,\n        \&quot;commune\&quot; : {\n          \&quot;codePostal\&quot; : \&quot;31000\&quot;,\n          \&quot;commune\&quot; : \&quot;Toulouse\&quot;,\n          \&quot;pays\&quot; : \&quot;FR\&quot;,\n          \&quot;codeInsee\&quot; : \&quot;31555\&quot;\n        }\n      },\n      \&quot;projetTerrestre\&quot; : true,\n      \&quot;projetMaritimeFluvial\&quot; : false,\n      \&quot;coordonneeXAIOT\&quot; : \&quot;569004\&quot;,\n      \&quot;coordonneeYAIOT\&quot; : \&quot;6274560\&quot;,\n      \&quot;systemeCoordonneesAIOT\&quot; : \&quot;P001\&quot;,\n      \&quot;precisionAIOT\&quot; : 1,\n      \&quot;fichierParcelles\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierParcelles.csv\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.csv\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;041\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;A4R6IcX965Oac8IXseFyDd+0x4K5Nu/Hh/b+pvS7UzY\u003d\&quot;\n      },\n      \&quot;fichierJustificatifMaitriseFonciere\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierJustificatifMaitriseFonciere.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;004\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;demandeComprendRubriqueIOTAouICPE\&quot; : 1,\n      \&quot;rubriquesNomenclatures\&quot; : [ {\n        \&quot;numeroRubrique\&quot; : \&quot;2.1.5.0\&quot;,\n        \&quot;alinea\&quot; : \&quot;1\&quot;,\n        \&quot;designationRubrique\&quot; : \&quot;Rejets d\u0027\u0027eaux pluviales\&quot;,\n        \&quot;quantiteProjet\&quot; : \&quot;22\&quot;,\n        \&quot;quantiteTotale\&quot; : \&quot;22\&quot;,\n        \&quot;unite\&quot; : \&quot;ha\&quot;,\n        \&quot;regime\&quot; : \&quot;A\&quot;,\n        \&quot;precisionRubrique\&quot; : \&quot;\&quot;,\n        \&quot;typeRubrique\&quot; : \&quot;IOTA\&quot;\n      } ],\n      \&quot;soumisEvaluationEnvironnementale\&quot; : true,\n      \&quot;rubriquesEEDila\&quot; : [ {\n        \&quot;regimeEE\&quot; : 1,\n        \&quot;numeroCategorieSousCategorieEE\&quot; : \&quot;1° a)\&quot;,\n        \&quot;libelleCategorieSousCategorieEE\&quot; : \&quot; Installations classées mentionnées à l’article L.515-28 du CE\&quot;\n      } ],\n      \&quot;demandeComprendEtudeImpactOuEtudeIncidence\&quot; : \&quot;1\&quot;,\n      \&quot;demandeEtudeImpactRaison\&quot; : 1,\n      \&quot;casParCasTacite\&quot; : false,\n      \&quot;presentationObjetProjet\&quot; : \&quot;test AUTO snd\&quot;,\n      \&quot;demandeComprendDeclarationInteretGeneralIOTA\&quot; : false,\n      \&quot;demandeComprendPrelevementEauIOTA\&quot; : false,\n      \&quot;demandeComprendAucuneCaracteristiquesIOTA\&quot; : true,\n      \&quot;fichierPlan25000ou50000\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierPlan25000ou50000.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;029\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;fichierElementsGraphiques\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierElementsGraphiques.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;030\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;commentaireGeneraleTeleprocedure\&quot; : \&quot;Test AUTO snd\&quot;,\n      \&quot;fichierAccuseReception\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierAccuseReception.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;045\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;${date_validation}\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 0,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;fichierEtudeImpact\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierEtudeImpact.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;006\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;fichierEtudeImpactAnnexes\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierEtudeImpactAnnexes.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;007\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;fichierResumeNontechniqueEtudeImpact\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierResumeNontechniqueEtudeImpact.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;008\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;2022-09-30T11:19:52\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 3,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;fichierSyntheseDepotTeleprocedure\&quot; : {\n        \&quot;numeroTeledemarche\&quot; : \&quot;${numero_teledemarche}\&quot;,\n        \&quot;nomFichier\&quot; : \&quot;fichierSyntheseDepotTeleprocedure.pdf\&quot;,\n        \&quot;nomOriginalFichier\&quot; : \&quot;aaa.pdf\&quot;,\n        \&quot;typeFichier\&quot; : \&quot;058\&quot;,\n        \&quot;dateDepot\&quot; : \&quot;${date_validation}\&quot;,\n        \&quot;statutFichier\&quot; : 0,\n        \&quot;etatFichier\&quot; : 0,\n        \&quot;checkSum\&quot; : \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n      },\n      \&quot;demandeRegularisarionActivite\&quot; : false,\n      \&quot;version\&quot; : 0\n    }\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>080461c6-d37f-4bd3-bac6-015c44ca900c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>92491d5c-a6f0-4768-b77b-a7b78a64ef6f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>7.8.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${serverHost}/dossier/depot/dossier</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.serverHost</defaultValue>
      <description></description>
      <id>f6f9a331-c723-4061-82f4-3b913c69a314</id>
      <masked>false</masked>
      <name>serverHost</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>cdb7b9ff-3845-4770-b835-1476d89496b5</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'PROJET-ANV-005'</defaultValue>
      <description></description>
      <id>490e77dc-2992-4d8e-8e3b-750193fdcda6</id>
      <masked>false</masked>
      <name>nom_projet</name>
   </variables>
   <variables>
      <defaultValue>'KTL-ANV-005'</defaultValue>
      <description></description>
      <id>7484a660-1309-493b-9eb3-2ac316fc029c</id>
      <masked>false</masked>
      <name>numero_teledemarche</name>
   </variables>
   <variables>
      <defaultValue>'2022-10-02T24:00:00'</defaultValue>
      <description></description>
      <id>c695377a-c222-411b-b9ce-c1b25cb0c99e</id>
      <masked>false</masked>
      <name>date_validation</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
