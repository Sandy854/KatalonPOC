import org.apache.commons.io.FilenameUtils


String textOne="bloubla"
String nomFichier="DescriptionProjet.pdf"
String ext=FilenameUtils.getExtension(nomFichier)
println("extension est : "+ext)


