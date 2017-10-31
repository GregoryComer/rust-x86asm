use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 223], OperandSize::Dword)
}

#[test]
fn movapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 54], OperandSize::Dword)
}

#[test]
fn movapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 218], OperandSize::Qword)
}

#[test]
fn movapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 134780835, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 132, 131, 163, 151, 8, 8], OperandSize::Qword)
}

#[test]
fn movapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 200], OperandSize::Dword)
}

#[test]
fn movapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 63], OperandSize::Dword)
}

#[test]
fn movapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 198], OperandSize::Qword)
}

#[test]
fn movapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1644155534, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 180, 153, 142, 210, 255, 97], OperandSize::Qword)
}

