use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 198], OperandSize::Dword)
}

#[test]
fn vrcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1629805944, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 156, 143, 120, 221, 36, 97], OperandSize::Dword)
}

#[test]
fn vrcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 253], OperandSize::Qword)
}

#[test]
fn vrcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1814801279, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 164, 75, 127, 171, 43, 108], OperandSize::Qword)
}

#[test]
fn vrcpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 244], OperandSize::Dword)
}

#[test]
fn vrcpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 401157678, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 12, 93, 46, 46, 233, 23], OperandSize::Dword)
}

#[test]
fn vrcpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 228], OperandSize::Qword)
}

#[test]
fn vrcpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 15], OperandSize::Qword)
}

