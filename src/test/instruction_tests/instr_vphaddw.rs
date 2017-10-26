use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 1, 220], OperandSize::Dword)
}

#[test]
fn vphaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 1, 9], OperandSize::Dword)
}

#[test]
fn vphaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 1, 213], OperandSize::Qword)
}

#[test]
fn vphaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 1, 52, 67], OperandSize::Qword)
}

#[test]
fn vphaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 1, 224], OperandSize::Dword)
}

#[test]
fn vphaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 607303312, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 1, 4, 141, 144, 182, 50, 36], OperandSize::Dword)
}

#[test]
fn vphaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 1, 227], OperandSize::Qword)
}

#[test]
fn vphaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1866148078, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 1, 156, 89, 238, 40, 59, 111], OperandSize::Qword)
}

