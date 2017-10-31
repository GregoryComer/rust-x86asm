use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 125, 205], OperandSize::Dword)
}

#[test]
fn vhsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 764930161, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 125, 4, 149, 113, 232, 151, 45], OperandSize::Dword)
}

#[test]
fn vhsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 125, 232], OperandSize::Qword)
}

#[test]
fn vhsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 2020932952, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 125, 188, 80, 88, 253, 116, 120], OperandSize::Qword)
}

#[test]
fn vhsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 125, 242], OperandSize::Dword)
}

#[test]
fn vhsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 125, 7], OperandSize::Dword)
}

#[test]
fn vhsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 125, 248], OperandSize::Qword)
}

#[test]
fn vhsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 395744593, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 125, 174, 81, 149, 150, 23], OperandSize::Qword)
}

