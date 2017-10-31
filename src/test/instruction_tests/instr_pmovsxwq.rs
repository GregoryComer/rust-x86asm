use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 197], OperandSize::Dword)
}

#[test]
fn pmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1996341674, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 172, 246, 170, 193, 253, 118], OperandSize::Dword)
}

#[test]
fn pmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 235], OperandSize::Qword)
}

#[test]
fn pmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1366967259, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 44, 213, 219, 67, 122, 81], OperandSize::Qword)
}

