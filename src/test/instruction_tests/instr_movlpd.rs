use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ECX, 1627473663, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 153, 255, 70, 1, 97], OperandSize::Dword)
}

#[test]
fn movlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 36, 143], OperandSize::Qword)
}

#[test]
fn movlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 28, 243], OperandSize::Dword)
}

#[test]
fn movlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(IndirectScaledDisplaced(RSI, Four, 2118649967, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 12, 181, 111, 8, 72, 126], OperandSize::Qword)
}

