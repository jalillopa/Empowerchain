# 🌱 EmpowerChain: Protocolo de Microfinanzas Descentralizadas con Scoring IA y Trazabilidad ESG

EmpowerChain conecta microemprendedores no bancarizados con inversionistas de impacto mediante smart contracts, mitigación de riesgo por hitos y auditoría de métricas ESG en tiempo real.

## 🚀 Enlaces Clave
- **Demo Interactiva**: [Desplegar en Vercel o GitHub Pages]
- **Red de Despliegue**: Arbitrum Stylus (Layer 2)
- **Lenguaje del Contrato**: Rust (WASM)

## 🧩 Características del Protocolo
1. **Términos Claros**: Visualización previa de tasas fijas, comisiones y calendario.
2. **Hitos Configurables**: Liberación fraccionada del capital sujeta a validación técnica on-chain.
3. **Gestión de Impago (Default)**: Reintegro automático de fondos no desembolsados en custodia.
4. **Métricas ESG**: Registro inmutable de impacto ambiental y reducción estimada de CO2.

## 🛠️ Estructura del Repositorio
- `/contracts`: Código fuente del Smart Contract en Rust (`EmpowerChainEngine.rs`).
- `/demo`: Prototipo de interfaz web interactiva (`index.html`).

## 🚀 Instrucciones de Despliegue (Arbitrum Stylus / Testnet)

Para compilar y desplegar el contrato inteligente en Rust:

```bash
# 1. Clonar el repositorio
git clone [https://github.com/jalillopa/Empowerchain.git](https://github.com/jalillopa/Empowerchain.git)
cd Empowerchain

# 2. Configurar el compilador WASM y Stylus CLI
rustup target add wasm32-unknown-unknown
## 📜 Smart Contract & Despliegue

El contrato inteligente central del protocolo (`EmpowerChainEngine.sol`) gestiona la custodia de fondos, liberación por hitos y protección ante default en la red Arbitrum.

### Probar y desplegar en Remix IDE

Puedes abrir, compilar y desplegar el contrato directamente en Remix haciendo clic en el siguiente botón:

[![Open in Remix](https://img.shields.io/badge/Open%20in-Remix-blue?logo=ethereum)](https://remix.ethereum.org/#gist=32ddcee9769a6485f89413034e22fcc2)

* **Enlace directo a Remix:** [Abrir EmpowerChainEngine en Remix IDE](https://remix.ethereum.org/#gist=32ddcee9769a6485f89413034e22fcc2)
* **Código fuente en Gist:** [Ver Gist en GitHub](https://gist.github.com/jalillopa/32ddcee9769a6485f89413034e22fcc2)

### Instrucciones de despliegue rápido:
1. Haz clic en el botón **Open in Remix**.
2. En Remix, ve a la pestaña **Solidity Compiler** y compila `EmpowerChainEngine.sol`.
3. Ve a **Deploy & Run Transactions**, selecciona en *Environment* tu billetera (**Browser Extension / MetaMask**).
4. Elige la red (**Arbitrum One** o **Arbitrum Sepolia**), mantén el campo *Value* en `0` y presiona **Deploy**.
cargo install cargo-stylus

# 3. Verificar el contrato
cargo stylus check

# 4. Desplegar en Arbitrum Sepolia
cargo stylus deploy --private-key=<TU_PRIVATE_KEY> --endpoint=[https://sepolia-rollup.arbitrum.io/rpc](https://sepolia-rollup.arbitrum.io/rpc)
