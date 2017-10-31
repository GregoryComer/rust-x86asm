use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 125, 215], OperandSize::Dword)
}

#[test]
fn vhsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 125, 28, 89], OperandSize::Dword)
}

#[test]
fn vhsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 125, 219], OperandSize::Qword)
}

#[test]
fn vhsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 1261195106, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 125, 132, 243, 98, 79, 44, 75], OperandSize::Qword)
}

#[test]
fn vhsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 223, 125, 231], OperandSize::Dword)
}

#[test]
fn vhsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 183200699, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 223, 125, 4, 125, 187, 107, 235, 10], OperandSize::Dword)
}

#[test]
fn vhsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 239, 125, 218], OperandSize::Qword)
}

#[test]
fn vhsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 406447413, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 231, 125, 132, 208, 53, 229, 57, 24], OperandSize::Qword)
}

