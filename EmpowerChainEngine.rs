use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LoanStatus {
    PendingFunding,
    Active,
    Defaulted,
    FullyRepaid,
}

#[derive(Debug, Clone)]
pub struct Milestone {
    pub id: u32,
    pub description: String,
    pub amount_percentage: u8, // e.g., 40%
    pub is_verified: bool,
    pub is_disbursed: bool,
}

#[derive(Debug, Clone)]
pub struct LoanAgreement {
    pub loan_id: String,
    pub entrepreneur: String,
    pub investor: String,
    pub principal_amount: u64,
    pub interest_rate_basis_points: u16, // 1000 = 10.00%
    pub platform_fee_basis_points: u16,  // 300 = 3.00%
    pub esg_score: u8,                   // 0 a 100
    pub carbon_impact_tons: u64,         // Métricas ESG on-chain
    pub milestones: Vec<Milestone>,
    pub disbursed_amount: u64,
    pub repaid_amount: u64,
    pub status: LoanStatus,
}

pub struct EmpowerChainContract {
    pub owner: String,
    pub loans: HashMap<String, LoanAgreement>,
    pub total_impact_carbon: u64,
}

impl EmpowerChainContract {
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            loans: HashMap::new(),
            total_impact_carbon: 0,
        }
    }

    /// Inicializa un préstamo con hitos configurables y reglas claras
    pub fn create_loan_request(
        &mut self,
        loan_id: String,
        entrepreneur: String,
        investor: String,
        principal_amount: u64,
        interest_rate_bps: u16,
        esg_score: u8,
        milestones: Vec<Milestone>,
    ) -> Result<(), &'static str> {
        let total_pct: u8 = milestones.iter().map(|m| m.amount_percentage).sum();
        if total_pct != 100 {
            return Err("La suma de los porcentajes de los hitos debe ser exactamente 100%");
        }

        // Estimación de carbono: (principal * esg_score) / 10000
        let carbon_impact = (principal_amount * esg_score as u64) / 10000;

        let loan = LoanAgreement {
            loan_id: loan_id.clone(),
            entrepreneur,
            investor,
            principal_amount,
            interest_rate_basis_points: interest_rate_bps,
            platform_fee_basis_points: 300, // 3% comisión de plataforma
            esg_score,
            carbon_impact_tons: carbon_impact,
            milestones,
            disbursed_amount: 0,
            repaid_amount: 0,
            status: LoanStatus::PendingFunding,
        };

        self.loans.insert(loan_id, loan);
        self.total_impact_carbon += carbon_impact;
        Ok(())
    }

    /// Fondear el préstamo por el inversionista
    pub fn fund_loan(&mut self, loan_id: &str) -> Result<(), &'static str> {
        let loan = self.loans.get_mut(loan_id).ok_or("Préstamo no encontrado")?;
        if loan.status != LoanStatus::PendingFunding {
            return Err("El préstamo no está pendiente de financiamiento");
        }
        loan.status = LoanStatus::Active;
        Ok(())
    }

    /// Liberar fondos por cumplimiento de hito
    pub fn release_milestone(&mut self, loan_id: &str, milestone_id: u32) -> Result<u64, &'static str> {
        let loan = self.loans.get_mut(loan_id).ok_or("Préstamo no encontrado")?;
        if loan.status != LoanStatus::Active {
            return Err("El préstamo no está activo");
        }

        let milestone = loan
            .milestones
            .iter_mut()
            .find(|m| m.id == milestone_id)
            .ok_or("Hito no existe")?;

        if !milestone.is_verified {
            return Err("El hito aún no ha sido verificado en campo/auditoría");
        }
        if milestone.is_disbursed {
            return Err("Este hito ya fue desembolsado");
        }

        let tranche = (loan.principal_amount * milestone.amount_percentage as u64) / 100;
        milestone.is_disbursed = true;
        loan.disbursed_amount += tranche;

        Ok(tranche)
    }

    /// Manejo de default/incumplimiento: protege el remanente no liberado
    pub fn trigger_default(&mut self, loan_id: &str) -> Result<u64, &'static str> {
        let loan = self.loans.get_mut(loan_id).ok_or("Préstamo no encontrado")?;
        if loan.status != LoanStatus::Active {
            return Err("El préstamo no se encuentra en estado activo");
        }

        loan.status = LoanStatus::Defaulted;
        let unreleased_funds = loan.principal_amount - loan.disbursed_amount;
        
        // Los fondos no desembolsados quedan protegidos y listos para reintegrarse al inversionista
        Ok(unreleased_funds)
    }
}