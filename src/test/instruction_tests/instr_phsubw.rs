use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 192], OperandSize::Dword)
}

#[test]
fn phsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 9], OperandSize::Dword)
}

#[test]
fn phsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 207], OperandSize::Qword)
}

#[test]
fn phsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1794063046, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 60, 125, 198, 58, 239, 106], OperandSize::Qword)
}

#[test]
fn phsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 237], OperandSize::Dword)
}

#[test]
fn phsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 929207409, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 156, 135, 113, 148, 98, 55], OperandSize::Dword)
}

#[test]
fn phsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 241], OperandSize::Qword)
}

#[test]
fn phsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 374711085, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 188, 81, 45, 163, 85, 22], OperandSize::Qword)
}

