string=$(sudo ./get_db.sh)

filename=".env"
expected_start="DATABASE_URL=postgres"
first_line=$(head -n 1 "$filename")

function write_string {
   printf "DATABASE_URL=%s" "$string" > .env
}

if [[ ! -f "$filename" ]] || [[ -s "$filename" ]]; then
   write_string
   exit 0
fi

# Check if the file has only one line (using wc -l) and if it starts with the expected string
if [[ $(wc -l < "$filename") -ne 1 || "$first_line" != "$expected_start"* ]]; then
   printf "ANTI FOOT-GUN: It looks like .env is not one line long or does not start with 'DATABASE_URL=postgres'\n" 1>&2
   exit 1
fi

write_string
