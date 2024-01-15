#!/bin/bash
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NO_COLOR='\033[0m'

# Reset
Color_Off='\033[0m'	   # Text Reset //https://dev.to/ifenna__/adding-colors-to-bash-scripts-48g4

# Regular Colors
Black='\033[0;30m'		# Black
Red='\033[0;31m'		  # Red
Green='\033[0;32m'		# Green
Yellow='\033[0;33m'	   # Yellow
Blue='\033[0;34m'		 # Blue
Purple='\033[0;35m'	   # Purple
Cyan='\033[0;36m'		 # Cyan
White='\033[0;37m'		# White

# Bold
BBlack='\033[1;30m'	   # Black
BRed='\033[1;31m'		 # Red
BGreen='\033[1;32m'	   # Green
BYellow='\033[1;33m'	  # Yellow
BBlue='\033[1;34m'		# Blue
BPurple='\033[1;35m'	  # Purple
BCyan='\033[1;36m'		# Cyan
BWhite='\033[1;37m'	   # White

# Underline
UBlack='\033[4;30m'	   # Black
URed='\033[4;31m'		 # Red
UGreen='\033[4;32m'	   # Green
UYellow='\033[4;33m'	  # Yellow
UBlue='\033[4;34m'		# Blue
UPurple='\033[4;35m'	  # Purple
UCyan='\033[4;36m'		# Cyan
UWhite='\033[4;37m'	   # White

# Background
On_Black='\033[40m'	   # Black
On_Red='\033[41m'		 # Red
On_Green='\033[42m'	   # Green
On_Yellow='\033[43m'	  # Yellow
On_Blue='\033[44m'		# Blue
On_Purple='\033[45m'	  # Purple
On_Cyan='\033[46m'		# Cyan
On_White='\033[47m'	   # White

On_White='\033[1;107m'	   # White

# High Intensity
IBlack='\033[0;90m'	   # Black
IRed='\033[0;91m'		 # Red
IGreen='\033[0;92m'	   # Green
IYellow='\033[0;93m'	  # Yellow
IBlue='\033[0;94m'		# Blue
IPurple='\033[0;95m'	  # Purple
ICyan='\033[0;96m'		# Cyan
IWhite='\033[0;97m'	   # White

# Bold High Intensity
BIBlack='\033[1;90m'	  # Black
BIRed='\033[1;91m'		# Red
BIGreen='\033[1;92m'	  # Green
BIYellow='\033[1;93m'	 # Yellow
BIBlue='\033[1;94m'	   # Blue
BIPurple='\033[1;95m'	 # Purple
BICyan='\033[1;96m'	   # Cyan
BIWhite='\033[1;97m'	  # White

# High Intensity backgrounds
On_IBlack='\033[0;100m'   # Black
On_IRed='\033[0;101m'	 # Red
On_IGreen='\033[0;102m'   # Green
On_IYellow='\033[0;103m'  # Yellow
On_IBlue='\033[0;104m'	# Blue
On_IPurple='\033[0;105m'  # Purple
On_ICyan='\033[0;106m'	# Cyan
On_IWhite='\033[0;107m'   # White

log(){
  local color="$1" # First argument
  local message="$2" # Second argument
  echo -e "${color}${message}${NO_COLOR}"
}

retry() {
	local total="$1" # First argument
	local retries="$2" # First argument
	local command="$3" # Second argument
	local message="$4" # Second argument
	log "${BBlue}" "RUNNING:\\t${message}"
	$command # Run the command, and save the exit code
	local exit_code=$?

	if [[ $exit_code -ne 0 ]]; then 
		log "${RED}" "ERROR:\\t${message} ExitCode:${exit_code}\\n"
		if [[ $retries -eq 0 ]]; then 
			exit $exit_code
		fi
	else 
		log "${GREEN}" "SUCCESS:\\t${message} ExitCode:${exit_code}\\n"
	fi
	if [[ $exit_code -ne 0 && $retries -gt 0 ]]; then # If the exit code is non-zero (i.e. command failed), and we have not reached the maximum number of retries, run the command again
		log "${YELLOW}" "RETRY:\\t${message} :\\t retry $((total-retries+1))/$total"
		retry $total $(($retries - 1)) "$command" "$message"
	else # Return the exit code from the command
		return $exit_code
	fi
}

print_help() { # Help message function
    echo "Usage: $0 [command] [options]"
    echo
    echo "Commands:"
    echo "  clean        Remove all files and clone repositories"
    echo "  pull         update all repositories"
    echo "  start        Start DFX, optionally with bitcoin support"
    echo "  predeploy    Run pre-deployment scripts"
    echo "  deploy       Build & Deploy the application"
	echo "  system       Build & Deploy the B3System & Start in development mode"
    echo "  --help       Display this help message"
    echo
    echo "Options for 'start':"
    echo "  bitcoin      Enable bitcoin support in DFX"
    echo
    echo "Example:"
    echo "  $0 deploy         # Deploy the application"
    echo "  $0 start bitcoin  # Start DFX with bitcoin support"
}

if [ -z "$1" ] || [ "$1" = "--help" ]; then
	print_help
	exit 0
fi

CWD=$PWD
if [ "$1" = "clean" ]
then
	rm -rf * .* 
	git clone https://github.com/B3Pay/B3Wallet.git .
	git clone https://github.com/B3Pay/b3_utils.git ./backend/lib/b3_utils
  exit
fi

if [ "$1" = "pull" ]
then
	echo $PWD
	git pull
	cd $CWD
	cd ./backend/lib/b3_utils
	echo $PWD
	git pull
	cd $CWD
fi

if [ "$1" = "start" ]
then

	if [ "$2" = "bitcoin" ] ; then
		log "${On_White}" "Starting DFX with bitcoin"
		dfx start --enable-bitcoin --clean
	else
		log "${On_White}" "Starting DFX without bitcoin"
		dfx start --clean
	fi 
fi

if [ "$1" = "predeploy" ]
then
	sh scripts/predeploy.sh
fi

if [ "$1" = "deploy" ]
then
	log "${On_White}" "BUILDING:\\t B3 \\n"
	retry 1 0 "dfx deps pull" 								"Pulling B3 Dependencies"
	retry 1 0 "dfx deps init" 								"Initializing B3 Dependencies"
	retry 1 0 "dfx deps deploy internet_identity" 			"Deploying Internet-Identity Canister"
	retry 1 0 "bash scripts/install.sh" 					"Installing ckbtc, kyt, minter, and index"
	retry 1 0 "yarn install" 								"Installing NodeJS dependencies"
	retry 1 1 "dfx deploy" 									"Deploying B3 on Network"
	retry 1 0 "dfx generate" 								"Generating Candid for B3"
	retry 1 0 "source .env" 								"Loading DFX Environment"
	retry 1 0 "npx ts-node scripts/load-wasm.system.ts" 	"Uploading B3 System WASM"
	cd $CWD
fi

if [ "$1" = "system" ]
then
	log "${On_White}" "BUILDING:\\t B3System \\n"
	retry 1 0 "dfx deps pull" 								"Pulling B3 Dependencies"
	retry 1 0 "dfx deps init" 								"Initializing B3 Dependencies"
	retry 1 0 "dfx deps deploy internet_identity" 			"Deploying Internet-Identity Canister"
	retry 1 0 "yarn install" 								"Installing NodeJS dependencies"
	retry 1 0 "yarn predeploy" 								"Predeploying all canisters"
	retry 1 1 "dfx deploy b3system" 						"Deploying B3System on Network"
	retry 1 0 "dfx generate b3system" 						"Generating Candid for B3"
	retry 1 0 "source .env" 								"Loading DFX Environment"
	retry 1 0 "yarn dev" 									"Starting B3System in development mode"
	cd $CWD
fi
cd $CWD
unset CWD