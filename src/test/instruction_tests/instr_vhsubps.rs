use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 125, 220], OperandSize::Dword)
}

#[test]
fn vhsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 1086321132, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 125, 155, 236, 241, 191, 64], OperandSize::Dword)
}

#[test]
fn vhsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 125, 216], OperandSize::Qword)
}

#[test]
fn vhsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 804659195, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 125, 52, 85, 251, 31, 246, 47], OperandSize::Qword)
}

#[test]
fn vhsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 125, 226], OperandSize::Dword)
}

#[test]
fn vhsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 125, 28, 89], OperandSize::Dword)
}

#[test]
fn vhsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 125, 245], OperandSize::Qword)
}

#[test]
fn vhsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1635315692, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 125, 12, 77, 236, 239, 120, 97], OperandSize::Qword)
}

