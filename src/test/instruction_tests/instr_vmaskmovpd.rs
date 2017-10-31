use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 45, 52, 218], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 45, 46], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 875356580, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 45, 137, 164, 225, 44, 52], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 1825321, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 45, 170, 41, 218, 27, 0], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1752005954, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 47, 28, 197, 66, 125, 109, 104], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectDisplaced(RBX, 1697815709, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 47, 147, 157, 156, 50, 101], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 220319468, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 47, 36, 213, 236, 206, 33, 13], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledDisplaced(RAX, Four, 805767182, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 47, 4, 133, 14, 8, 7, 48], OperandSize::Qword)
}

