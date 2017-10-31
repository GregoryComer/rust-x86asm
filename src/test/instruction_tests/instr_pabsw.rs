use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 213], OperandSize::Dword)
}

#[test]
fn pabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1634984716, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 140, 119, 12, 227, 115, 97], OperandSize::Dword)
}

#[test]
fn pabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 205], OperandSize::Qword)
}

#[test]
fn pabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 776339891, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 28, 85, 179, 1, 70, 46], OperandSize::Qword)
}

#[test]
fn pabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 201], OperandSize::Dword)
}

#[test]
fn pabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 28, 185], OperandSize::Dword)
}

#[test]
fn pabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 243], OperandSize::Qword)
}

#[test]
fn pabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1349647848, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 36, 181, 232, 253, 113, 80], OperandSize::Qword)
}

