use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1331721000, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 44, 132, 89, 40, 115, 96, 79], OperandSize::Dword)
}

#[test]
fn vmaskmovps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 44, 60, 178], OperandSize::Qword)
}

#[test]
fn vmaskmovps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 44, 8], OperandSize::Dword)
}

#[test]
fn vmaskmovps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1601258392, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 44, 20, 253, 152, 67, 113, 95], OperandSize::Qword)
}

#[test]
fn vmaskmovps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 46, 39], OperandSize::Dword)
}

#[test]
fn vmaskmovps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 312335402, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 46, 172, 183, 42, 220, 157, 18], OperandSize::Qword)
}

#[test]
fn vmaskmovps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 578529797, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 46, 180, 242, 5, 170, 123, 34], OperandSize::Dword)
}

#[test]
fn vmaskmovps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 46, 33], OperandSize::Qword)
}

