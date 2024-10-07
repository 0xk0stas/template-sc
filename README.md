<sub>*This file has been auto-generated using the [abi-markdowner](https://github.com/0xk0stas/abi-markdowner).*</sub>

# Smart Contract: NftUpgradeModule

<details>
<summary>Build info</summary>

- **Rustc Version**: 1.79.0
- **Commit Hash**: 129f3b9964af4d4a709d1383930ade12dfe7c081
- **Commit Date**: 2024-06-10
- **Channel**: Stable

- **Framework**: multiversx-sc
- **Version**: 0.53.2
</details>

## Table of Contents

- [Endpoints](#endpoints)
- [Views](#views)

## Endpoints

### Deploy

<details>
<summary>init</summary>


</details>

### Upgrade

<details>
<summary>upgrade</summary>


</details>

### Owner Only

<details>
<summary>upgradeNft</summary>

upgrade_nft

receive the NFT,

read the attributes,

identify the level and increase it
#### Inputs:
| Name | Type |
| - | - |
| user | Address |


</details>

<details>
<summary>resetNft</summary>

#### Inputs:
| Name | Type |
| - | - |
| user | Address |


</details>

<details>
<summary>pauseSc</summary>


</details>

<details>
<summary>resumeSc</summary>


</details>

## Views

<details>
<summary>getEmrNft</summary>

#### Outputs:
| Type |
| - |
| TokenIdentifier |


</details>

<details>
<summary>getIsScPaused</summary>

#### Outputs:
| Type |
| - |
| bool |


</details>

