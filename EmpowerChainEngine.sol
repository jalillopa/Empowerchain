// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract EmpowerChainEngine {
    enum LoanState { Active, Completed, Defaulted }

    struct Loan {
        string entrepreneurId;
        address payable entrepreneurWallet;
        address payable investor;
        uint256 principal;
        uint256 remainingEscrow;
        uint256 interestRateApr;
        uint256 esgScore;
        uint256 co2MitigatedEst;
        bool milestone1Disbursed;
        bool milestone2Disbursed;
        LoanState state;
    }

    mapping(uint256 => Loan) public loans;
    uint256 public nextLoanId;

    event LoanCreated(uint256 indexed loanId, string entrepreneurId, uint256 principal, uint256 esgScore);
    event MilestoneDisbursed(uint256 indexed loanId, uint8 milestoneNumber, uint256 amount);
    event DefaultExecuted(uint256 indexed loanId, uint256 refundedAmount);
    event LoanCompleted(uint256 indexed loanId);

    function createLoan(
        string memory _entrepreneurId,
        address payable _entrepreneurWallet,
        uint256 _interestRateApr,
        uint256 _esgScore,
        uint256 _co2MitigatedEst
    ) external payable returns (uint256) {
        require(msg.value > 0, "El capital enviado debe ser mayor a 0");
        require(_esgScore <= 100, "Score ESG debe estar entre 0 y 100");

        uint256 loanId = nextLoanId++;

        loans[loanId] = Loan({
            entrepreneurId: _entrepreneurId,
            entrepreneurWallet: _entrepreneurWallet,
            investor: payable(msg.sender),
            principal: msg.value,
            remainingEscrow: msg.value,
            interestRateApr: _interestRateApr,
            esgScore: _esgScore,
            co2MitigatedEst: _co2MitigatedEst,
            milestone1Disbursed: false,
            milestone2Disbursed: false,
            state: LoanState.Active
        });

        emit LoanCreated(loanId, _entrepreneurId, msg.value, _esgScore);
        return loanId;
    }

    function releaseMilestone1(uint256 _loanId) external {
        Loan storage loan = loans[_loanId];
        require(loan.state == LoanState.Active, "El prestamo no esta activo");
        require(!loan.milestone1Disbursed, "Hito 1 ya desembolsado");

        uint256 disbursementAmount = loan.principal / 2;
        require(loan.remainingEscrow >= disbursementAmount, "Fondos insuficientes");

        loan.milestone1Disbursed = true;
        loan.remainingEscrow -= disbursementAmount;

        (bool success, ) = loan.entrepreneurWallet.call{value: disbursementAmount}("");
        require(success, "Fallo la transferencia");

        emit MilestoneDisbursed(_loanId, 1, disbursementAmount);
    }

    function releaseMilestone2(uint256 _loanId) external {
        Loan storage loan = loans[_loanId];
        require(loan.state == LoanState.Active, "El prestamo no esta activo");
        require(loan.milestone1Disbursed, "Debe completarse el Hito 1 primero");
        require(!loan.milestone2Disbursed, "Hito 2 ya desembolsado");

        uint256 disbursementAmount = loan.remainingEscrow;
        loan.milestone2Disbursed = true;
        loan.remainingEscrow = 0;
        loan.state = LoanState.Completed;

        (bool success, ) = loan.entrepreneurWallet.call{value: disbursementAmount}("");
        require(success, "Fallo la transferencia");

        emit MilestoneDisbursed(_loanId, 2, disbursementAmount);
        emit LoanCompleted(_loanId);
    }

    function triggerDefault(uint256 _loanId) external {
        Loan storage loan = loans[_loanId];
        require(loan.state == LoanState.Active, "El prestamo no esta activo");
        require(msg.sender == loan.investor, "Solo el inversionista puede activar el default");

        loan.state = LoanState.Defaulted;
        uint256 refundAmount = loan.remainingEscrow;
        loan.remainingEscrow = 0;

        if (refundAmount > 0) {
            (bool success, ) = loan.investor.call{value: refundAmount}("");
            require(success, "Fallo el reembolso");
        }

        emit DefaultExecuted(_loanId, refundAmount);
    }

    function getLoan(uint256 _loanId) external view returns (Loan memory) {
        return loans[_loanId];
    }
}
