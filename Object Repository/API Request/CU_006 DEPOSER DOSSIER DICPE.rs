<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CU_006 DEPOSER DOSSIER DICPE</name>
   <tag></tag>
   <elementGuidId>064bc14a-dcaa-4707-96ae-220755cae993</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;code\&quot;: \&quot;DILA\&quot;,\n    \&quot;numeroTeledemarche\&quot;: \&quot;${numero_teledemarche}\&quot;,\n    \&quot;metaDataDossier\&quot;: {\n        \&quot;numeroDossier\&quot;: \&quot;\&quot;,\n        \&quot;numeroDemarche\&quot;: \&quot;DICPE\&quot;,\n        \&quot;numeroTeledemarche\&quot;: \&quot;${numero_teledemarche}\&quot;,\n        \&quot;dateValidation\&quot;: \&quot;${date_validation}\&quot;,\n        \&quot;nombreFichiers\&quot;: 0,\n        \&quot;identificationDemande\&quot;: 1,\n        \&quot;orientationDemande\&quot;: {\n            \&quot;typeDemande\&quot;: 4,\n            \&quot;engagementFichiersDeposes\&quot;: true,\n            \&quot;engagementPrescriptionsGeneralesMinisterielles\&quot;: true,\n            \&quot;engagementPlans\&quot;: true,\n            \&quot;engagementDepotComplements\&quot;: true,\n            \&quot;codeAIOTInconnu\&quot;: true,\n            \&quot;installationNom\&quot;: \&quot;TEST\&quot;,\n            \&quot;serviceInstructeur\&quot;: \&quot;0\&quot;,\n            \&quot;petitionnaires\&quot;: [\n                {\n                    \&quot;numeroPetitionnaire\&quot;: \&quot;1\&quot;,\n                    \&quot;typePetitionnaire\&quot;: \&quot;2\&quot;,\n                    \&quot;personnePhysique\&quot;: {\n                        \&quot;civilite\&quot;: \&quot;1\&quot;,\n                        \&quot;nom\&quot;: \&quot;Doe\&quot;,\n                        \&quot;prenom\&quot;: \&quot;John\&quot;,\n                        \&quot;dateDeNaissance\&quot;: \&quot;2000-01-01\&quot;\n                    },\n                    \&quot;personnePhysiqueAnonyme\&quot;: false,\n                    \&quot;personnePhysiqueContact\&quot;: {\n                        \&quot;telephoneFixe\&quot;: \&quot;0123456789\&quot;,\n                        \&quot;telephoneMobile\&quot;: \&quot;0623456789\&quot;,\n                        \&quot;email\&quot;: \&quot;nom@exemple.com\&quot;\n                    },\n                    \&quot;petitionnaireAdresse\&quot;: {\n                        \&quot;adresse1\&quot;: \&quot;1 rue du Test\&quot;,\n                        \&quot;adresse2\&quot;: \&quot;\&quot;,\n                        \&quot;adresse3\&quot;: \&quot;\&quot;,\n                        \&quot;commune\&quot;: {\n                            \&quot;codePostal\&quot;: \&quot;75000\&quot;,\n                            \&quot;commune\&quot;: \&quot;aze\&quot;,\n                            \&quot;pays\&quot;: \&quot;AZ\&quot;,\n                            \&quot;codeInsee\&quot;: \&quot;75000\&quot;\n                        }\n                    }\n                }\n            ],\n            \&quot;projetNom\&quot;: \&quot;${nom_projet}\&quot;,\n            \&quot;emailEchangeAvecAdministration\&quot;: \&quot;snd.@yopmail.com\&quot;,\n            \&quot;etesVousPetitionnaire\&quot;: true,\n            \&quot;exploitationInstallationRegimeAutorisation\&quot;: false,\n            \&quot;liensInteractionsNouvelInstallAvecExistant\&quot;: \&quot;Liens et interactions de la nouvelle installation\&quot;,           \n            \&quot;adresseInstallation\&quot;: {\n                \&quot;adresse1\&quot;: \&quot;12 Rue Charles le Moult\&quot;,\n                \&quot;commune\&quot;: {\n                    \&quot;codePostal\&quot;: \&quot;29200\&quot;,\n                    \&quot;commune\&quot;: \&quot;Brest\&quot;,\n                    \&quot;pays\&quot;: \&quot;FR\&quot;,\n                    \&quot;codeInsee\&quot;: \&quot;29019\&quot;\n                }\n            },\n            \&quot;adressePrincipaleAIOT\&quot;: {\n                \&quot;adresse1\&quot;: \&quot;12 Rue Charles le Moult\&quot;,\n                \&quot;commune\&quot;: {\n                    \&quot;codePostal\&quot;: \&quot;29200\&quot;,\n                    \&quot;commune\&quot;: \&quot;Brest\&quot;,\n                    \&quot;pays\&quot;: \&quot;FR\&quot;,\n                    \&quot;codeInsee\&quot;: \&quot;29019\&quot;\n                }\n            },\n            \&quot;coordonneeXAIOT\&quot;: \&quot;147029\&quot;,\n            \&quot;coordonneeYAIOT\&quot;: \&quot;6837421\&quot;,\n            \&quot;systemeCoordonneesAIOT\&quot;: \&quot;P001\&quot;,\n            \&quot;necessitePermisConstruire\&quot; : false,\n            \&quot;rubriquesConcerneCessationActivite\&quot;: [\n                {\n                    \&quot;numeroRubrique\&quot;: \&quot;2.1.1.0\&quot;,\n                    \&quot;alinea\&quot;: \&quot;1\&quot;,\n                    \&quot;designationRubrique\&quot;: \&quot;Stations d\u0027épuration des agglomérations d\u0027assainissement ou dispositifs d\u0027assainissement non collectif devant traiter une charge brute de pollution organique au sens de l\u0027article R. 2224-6 du code général des collectivités territoriales :\&quot;,\n                    \&quot;quantiteProjet\&quot;: \&quot;1000\&quot;,\n                    \&quot;quantiteTotale\&quot;: \&quot;1000\&quot;,\n                    \&quot;unite\&quot;: \&quot;kg\&quot;,\n                    \&quot;regime\&quot;: \&quot;A\&quot;,\n                    \&quot;precisionRubrique\&quot;: \&quot;\&quot;,\n                    \&quot;typeRubrique\&quot;: \&quot;IOTA\&quot;\n                },\n                {\n                    \&quot;numeroRubrique\&quot;: \&quot;2910\&quot;,\n                    \&quot;alinea\&quot;: \&quot;A.1\&quot;,\n                    \&quot;designationRubrique\&quot;: \&quot;Combustion à l’exclusion des activités visées par les rubriques 2770, 2771, 2971 ou 2931 et des installations classées au titre de larubrique 3110 ou au titre d’autres rubriques de la nomenclature pour lesquelles la combustion participe à la fusion, la cuisson ou au traitement, en mélange avec les gaz de combustion, des matières entrantes\&quot;,\n                    \&quot;quantiteProjet\&quot;: \&quot;35\&quot;,\n                    \&quot;quantiteTotale\&quot;: \&quot;45\&quot;,\n                    \&quot;unite\&quot;: \&quot;MW\&quot;,\n                    \&quot;regime\&quot;: \&quot;E\&quot;,\n                    \&quot;precisionRubrique\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;numeroRubrique\&quot;: \&quot;2.1.3.0\&quot;,\n                    \&quot;alinea\&quot;: \&quot;2\&quot;,\n                    \&quot;designationRubrique\&quot;: \&quot;Epandage et stockage en vue d\u0027épandage de boues produites dans un ou plusieurs systèmes d\u0027assainissement collectif des eaux usées et installations d\u0027assainissement non collectif, la quantité de boues épandues dans l\u0027année présentant les caractéristiques suivantes : \&quot;,\n                    \&quot;quantiteProjet\&quot;: \&quot;10000\&quot;,\n                    \&quot;quantiteTotale\&quot;: \&quot;234567\&quot;,\n                    \&quot;unite\&quot;: \&quot;t/an\&quot;,\n                    \&quot;regime\&quot;: \&quot;A\&quot;,\n                    \&quot;precisionRubrique\&quot;: \&quot;\&quot;,\n                    \&quot;typeRubrique\&quot;: \&quot;IOTA\&quot;\n                }\n            ],\n            \&quot;dateMiseArretInstallation\&quot;: true,\n            \&quot;cessationTotale\&quot;: true,\n            \&quot;parcellesCessation\&quot;:[{\n                \&quot;codePostal\&quot;:\&quot;29200\&quot;,\n                \&quot;codeInsee\&quot; : \&quot;29019\&quot; ,\n                \&quot;commune\&quot;: \&quot;Brest\&quot;,\n                \&quot;section\&quot;: \&quot;AB\&quot;,\n                \&quot;numeroParcelle\&quot; :1,\n                \&quot;superficieParcelle\&quot;: \&quot;12 ha\&quot;\n            } ],\n            \&quot;evacuationProduitsDangereux\&quot;: \&quot;Commentaires\&quot;,\n            \&quot;limitationsAccesSite\&quot;: \&quot;Commentaires\&quot;,\n            \&quot;suppressionRisquesIncendieExplosion\&quot;: \&quot;Commentaires\&quot;,\n            \&quot;surveillanceEffetsSurEnvironnement\&quot;:\&quot;Commentaires\&quot;,\n            \&quot;engagementDeclarantInformationEcriteCessation\&quot;:true,\n            \&quot;engagementDeclarantDepotAttestationMiseEnSecurite\&quot;: true,\n            \&quot;presenceCombustiblesLiquidesSolides2910\&quot;:true,\n            \&quot;fichierAccuseReception\&quot;: {\n                \&quot;numeroTeledemarche\&quot;: \&quot;${numero_teledemarche}\&quot;,\n                \&quot;nomFichier\&quot;: \&quot;fichierAccuseReception.pdf\&quot;,\n                \&quot;nomOriginalFichier\&quot;: \&quot;DescriptionProjet.pdf\&quot;,\n                \&quot;typeFichier\&quot;: \&quot;277\&quot;,\n                \&quot;dateDepot\&quot;: \&quot;2022-08-09T11:30:00\&quot;,\n                \&quot;statutFichier\&quot;: 0,\n                \&quot;etatFichier\&quot;: 0,\n                \&quot;checkSum\&quot;: \&quot;+aQ+FEM8uHAUIN/3jWmXzHPjsoDOwlexV5USaRx7zZ8\u003d\&quot;\n            }\n\n        }\n    }\n}&quot;,
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
      <defaultValue>'PROJET-KTL-001'</defaultValue>
      <description></description>
      <id>490e77dc-2992-4d8e-8e3b-750193fdcda6</id>
      <masked>false</masked>
      <name>nom_projet</name>
   </variables>
   <variables>
      <defaultValue>'SND-DICPE-CESS001'</defaultValue>
      <description></description>
      <id>7484a660-1309-493b-9eb3-2ac316fc029c</id>
      <masked>false</masked>
      <name>numero_teledemarche</name>
   </variables>
   <variables>
      <defaultValue>'2022-08-27T14:38:00'</defaultValue>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
