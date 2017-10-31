use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 125, 194], OperandSize::Dword)
}

#[test]
fn vhsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1269885038, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 125, 172, 147, 110, 232, 176, 75], OperandSize::Dword)
}

#[test]
fn vhsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 125, 247], OperandSize::Qword)
}

#[test]
fn vhsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1345674767, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 125, 188, 128, 15, 94, 53, 80], OperandSize::Qword)
}

#[test]
fn vhsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 125, 204], OperandSize::Dword)
}

#[test]
fn vhsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 125, 44, 129], OperandSize::Dword)
}

#[test]
fn vhsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 125, 241], OperandSize::Qword)
}

#[test]
fn vhsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 125, 4, 95], OperandSize::Qword)
}

