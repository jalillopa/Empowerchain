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
cargo install cargo-stylus

# 3. Verificar el contrato
cargo stylus check

# 4. Desplegar en Arbitrum Sepolia
cargo stylus deploy --private-key=<TU_PRIVATE_KEY> --endpoint=[https://sepolia-rollup.arbitrum.io/rpc](https://sepolia-rollup.arbitrum.io/rpc)
