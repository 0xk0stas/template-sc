view() {
    if [ -n "$2" ]; then
        mxpy contract query $SC_ADDRESS --proxy $PROXY --function $1 --arguments $2
    else
        mxpy contract query $SC_ADDRESS --proxy $PROXY --function $1
    fi
}

changeOwnerAddress() {
    mxpy tx new \
    --receiver $SC_ADDRESS --recall-nonce --pem $OWNER_PEM \
    --gas-limit 10000000 --outfile ./reports/changeOwnerAddress.report.json \
    --send --value 0 --proxy $PROXY --chain $CHAIN_ID \
    --data ChangeOwnerAddress@$1
}


claimDeveloperRewards() {
    mxpy tx new \
    --receiver $SC_ADDRESS --recall-nonce --pem $OWNER_PEM \
    --gas-limit 10000000 --outfile ./reports/claimDeveloperRewards.report.json \
    --send --value 0 --proxy $PROXY --chain $CHAIN_ID \
    --data ClaimDeveloperRewards
}

deploy() {
    local GAS_LIMIT=${2:-20000000}  # Default gas limit is 20 million

    if [ -n "$1" ]; then
        ARGS="--arguments $1"
    else
        ARGS=""
    fi

    mxpy contract deploy --bytecode $SC_BYTECODE \
    --recall-nonce --pem $OWNER_PEM \
    --gas-limit $GAS_LIMIT --metadata-payable-by-sc \
    --send --outfile ./reports/deploy.report.json \
    --proxy $PROXY --chain $CHAIN_ID \
    $ARGS
}

upgrade() {
    local GAS_LIMIT=${2:-20000000}  # Default gas limit is 20 million

    if [ -n "$1" ]; then
        ARGS="--arguments $1"
    else
        ARGS=""
    fi
    mxpy contract upgrade $SC_ADDRESS --bytecode $SC_BYTECODE \
    --recall-nonce --pem $OWNER_PEM \
    --gas-limit $GAS_LIMIT --metadata-payable-by-sc \
    --send --outfile ./reports/upgrade.report.json \
    --proxy $PROXY --chain $CHAIN_ID \
    $ARGS
}

# ======================================================================

runTx() {
    local RECEIVER=${1:-$SC_ADDRESS} # Default receiver is the SC address
    local EGLD_VALUE=${2:-0}  # Default EGLD value is 0
    local ENDPOINT_NAME=${3:-""}  # Default endpoint name is empty
    local ARGUMENTS=${4:-""}  # Default arguments are empty
    local GAS_LIMIT=${5:-20000000}  # Default gas limit is 20 million
    local REPORT_FILE=${ENDPOINT_NAME:-"tx"} # Default report file is tx.report.json
    local OUTFILE="./reports/$REPORT_FILE.report.json" # Default outfile is ./reports/tx.report.json

    mxpy tx new \
    --receiver $RECEIVER --recall-nonce --pem $OWNER_PEM \
    --gas-limit $GAS_LIMIT --outfile $OUTFILE \
    --send --value $EGLD_VALUE --wait-result \
    --proxy $PROXY --chain $CHAIN_ID \
    --data $ENDPOINT_NAME$ARGUMENTS
}

simulateTx() {
    local RECEIVER=${1:-$SC_ADDRESS} # Default receiver is the SC address
    local EGLD_VALUE=${2:-0}  # Default EGLD value is 0
    local ENDPOINT_NAME=${3:-""}  # Default endpoint name is empty
    local ARGUMENTS=${4:-""}  # Default arguments are empty
    local GAS_LIMIT=${5:-20000000}  # Default gas limit is 20 million
    local REPORT_FILE=${ENDPOINT_NAME:-"tx"} # Default report file is tx.report.json
    local OUTFILE="./reports/tx.report.json" # Default outfile is ./reports/tx.report.json

    mxpy tx new \
    --receiver $RECEIVER --recall-nonce --pem $OWNER_PEM \
    --gas-limit $GAS_LIMIT --outfile $OUTFILE \
    --simulate --value $EGLD_VALUE --wait-result \
    --proxy $PROXY --chain $CHAIN_ID \
    --data $ENDPOINT_NAME$ARGUMENTS
}