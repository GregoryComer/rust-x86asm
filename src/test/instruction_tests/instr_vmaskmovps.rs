use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 649622502, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 44, 146, 230, 115, 184, 38], OperandSize::Dword)
}

#[test]
fn vmaskmovps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 2131300197, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 44, 52, 149, 101, 15, 9, 127], OperandSize::Qword)
}

#[test]
fn vmaskmovps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1074535569, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 44, 172, 127, 145, 28, 12, 64], OperandSize::Dword)
}

#[test]
fn vmaskmovps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 44, 19], OperandSize::Qword)
}

#[test]
fn vmaskmovps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 159534624, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 46, 140, 94, 32, 78, 130, 9], OperandSize::Dword)
}

#[test]
fn vmaskmovps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 46, 8], OperandSize::Qword)
}

#[test]
fn vmaskmovps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectDisplaced(ESI, 1839074292, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 46, 166, 244, 11, 158, 109], OperandSize::Dword)
}

#[test]
fn vmaskmovps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectDisplaced(RCX, 1677724005, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 46, 145, 101, 9, 0, 100], OperandSize::Qword)
}

