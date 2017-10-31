use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 1, 236], OperandSize::Dword)
}

#[test]
fn vphaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 1318963397, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 1, 137, 197, 200, 157, 78], OperandSize::Dword)
}

#[test]
fn vphaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 1, 215], OperandSize::Qword)
}

#[test]
fn vphaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 2094404283, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 1, 164, 137, 187, 18, 214, 124], OperandSize::Qword)
}

#[test]
fn vphaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 1, 227], OperandSize::Dword)
}

#[test]
fn vphaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 1, 56], OperandSize::Dword)
}

#[test]
fn vphaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 1, 243], OperandSize::Qword)
}

#[test]
fn vphaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1427978217, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 1, 4, 149, 233, 55, 29, 85], OperandSize::Qword)
}

