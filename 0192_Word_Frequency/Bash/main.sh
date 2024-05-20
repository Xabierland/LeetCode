# Write a bash script to calculate the frequency of each word in a text file words.txt.
# For simplicity sake, you may assume:
# - words.txt contains only lowercase characters and space ' ' characters.
# - Each word must consist of lowercase characters only.
# - Words are separated by one or more whitespace characters.

# Example:
# Assume that words.txt has the following content:
#     the day is sunny the the
#     the sunny is is
# Your script should output the following, sorted by descending frequency:
#     the 4
#     is 3
#     sunny 2
#     day 1

# Read from the file words.txt and output the word frequency list to stdout.

cat words.txt |                      # Lee el contenido del archivo
tr -s '[:space:]' '\n' |             # Convierte los espacios en saltos de línea
grep -v '^$' |                       # Filtra líneas vacías (si las hubiera)
sort |                               # Ordena las palabras para que uniq pueda contarlas
uniq -c |                            # Cuenta las ocurrencias de cada palabra
sort -rn |                           # Ordena el resultado por número en orden descendente
awk '{print $2 " " $1}'              # Invierte las columnas para mostrar la palabra seguida de la frecuencia
