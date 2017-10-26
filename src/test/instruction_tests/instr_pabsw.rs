use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 234], OperandSize::Dword)
}

#[test]
fn pabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 52, 184], OperandSize::Dword)
}

#[test]
fn pabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 202], OperandSize::Qword)
}

#[test]
fn pabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 44, 210], OperandSize::Qword)
}

#[test]
fn pabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 242], OperandSize::Dword)
}

#[test]
fn pabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 47], OperandSize::Dword)
}

#[test]
fn pabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 247], OperandSize::Qword)
}

#[test]
fn pabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1889081683, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 148, 70, 83, 25, 153, 112], OperandSize::Qword)
}

