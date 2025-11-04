#!/bin/bash
#
# SUMMARY
# 
# Downloads and decrypts notes, saving the file with the same naming pattern as the website.
#
# USAGE
#
# ./download_notes.sh <session_cookie_guid> <destination_dir> <event> <quest> <part>
#
# <session_cookie_guid> Your session cookie from the everybody.codes website
# <destination_dir> Directory to save the notes file, e.g. input/
# <event> Event or story, e.g. 2024 or 1
# <quest> Quest
# <part> Part from 1 to 3
#
# EXAMPLE
#
# ./download_notes.sh 9cfd3fe7-05e7-42ad-b45a-70fa1620e470 input 2024 1 1

# Exit on errors, undefined variables, and pipe failures.
set -euo pipefail

# If you modify this script then update user agent to your own repository details.
readonly USER_AGENT="https://github.com/maneatingape/everybody-codes-rust helper script"

# ANSI codes for pretty printing.
readonly BOLD="\033[1m"
readonly RED="\033[31m"
readonly RESET="\033[0m"

error() {
    local message="$1"
    echo -e "${RED}ERROR${RESET} ${message}" >&2
    exit 1
}

request() {
    local url="$1"
    local field="$2"
    
    # Include cookie and user agent with all requests.
    local response
    response=$(curl --silent --fail --cookie "$COOKIE" --user-agent "$USER_AGENT" "$url") || {
        error "HTTP request failed for URL \"${url}\""
    }
    
    # Extract specified field from JSON response, removing quotes and trailing newline.
    local result
    result=$(jq --join-output ".[\"${field}\"] // error" <<< "$response" 2>/dev/null) || {
        error "Field \"${field}\" missing from JSON response"
    }
    
    echo "$result"
}

# Check for required dependencies
for cmd in curl jq xxd openssl; do
    command -v "$cmd" &>/dev/null || error "Missing dependency \"${cmd}\""
done

# Check correct number of command line parameters.
if [[ $# -lt 5 ]]; then
    echo -e "${BOLD}USAGE${RESET}   ./download_notes.sh <session_cookie_guid> <destination_dir> <event> <quest> <part>"
    echo -e "${BOLD}EXAMPLE${RESET} ./download_notes.sh 9cfd3fe7-05e7-42ad-b45a-70fa1620e470 input 2024 1 1"
    exit 1
fi

# Validate arguments
[[ "$1" =~ ^[0-9a-fA-F-]{36}$ ]] || error "Session cookie should be a GUID"
[[ -d "$2" ]] || error "Destination directory does not exist"
[[ "$3" =~ ^[0-9]+$ ]] || error "Event must be a number"
[[ "$4" =~ ^[0-9]+$ ]] || error "Quest must be a number"
[[ "$5" =~ ^[1-3]$ ]] || error "Part must be 1, 2, or 3"

# Read session cookie, destination directory, event, quest and part from command line arguments.
readonly COOKIE="everybody-codes=$1"
readonly DESTINATION_DIR="$2"
readonly EVENT="$3"
readonly QUEST="$4"
readonly PART="$5"

# Retrieve seed parameter.
SEED=$(request "https://everybody.codes/api/user/me" "seed") || error "Unable to fetch seed parameter"
[[ "$SEED" != "0" ]] || error "Invalid seed parameter. Check that session cookie is valid"

# Retrieve encrypted JSON input notes, extracting hex encoded field for the specified part.
ENCRYPTED=$(request "https://everybody-codes.b-cdn.net/assets/$EVENT/$QUEST/input/$SEED.json" "$PART") || {
    error "Unable to read encrypted data. Check that event, quest and part are correct"
}

# Retrieve AES key, converting to hex encoding on a single line.
KEY=$(request "https://everybody.codes/api/event/$EVENT/quest/$QUEST" "key$PART" | xxd -p -c 0) || {
    error "Unable to read decryption key. Check that previous parts are solved"
}

# Format output path.
OUTPUT=$(printf "%s/everybody_codes_e%d_q%02d_p%d.txt" "$DESTINATION_DIR" "$EVENT" "$QUEST" "$PART")

# Decrypt input and write to output file.
xxd -p -r <<< "$ENCRYPTED" | openssl enc -d -aes-256-cbc -iv "${KEY:0:32}" -K "$KEY" -out "$OUTPUT" || {
    error "Unable to decrypt and write output file \"$OUTPUT\""
}

# Success
echo -e "Created ${BOLD}${OUTPUT}${RESET}"
