<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description>Bouton affecter qui porte le nom du téléprocedure en paramètre</description>
   <name>Bouton Affecter</name>
   <tag></tag>
   <elementGuidId>924ee52a-59d1-4fb1-a5ad-9c8c6849da9f</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//span[contains(text(),'${nom_teleprocedure}')]//following::a[@title='Affecter'][1]</value>
      </entry>
      <entry>
         <key>BASIC</key>
         <value>//*[@nom_teleprocedure = '${nom_teleprocedure}']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>nom_teleprocedure</name>
      <type>Main</type>
      <value>${nom_teleprocedure}</value>
      <webElementGuid>75f26183-a61c-4a7c-a01b-878b90dab980</webElementGuid>
   </webElementProperties>
</WebElementEntity>
