use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 3, 240], OperandSize::Dword)
}

#[test]
fn vphaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 684773277, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 3, 156, 88, 157, 207, 208, 40], OperandSize::Dword)
}

#[test]
fn vphaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 3, 231], OperandSize::Qword)
}

#[test]
fn vphaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 495354088, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 3, 129, 232, 128, 134, 29], OperandSize::Qword)
}

#[test]
fn vphaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 3, 207], OperandSize::Dword)
}

#[test]
fn vphaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 3, 49], OperandSize::Dword)
}

#[test]
fn vphaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 3, 203], OperandSize::Qword)
}

#[test]
fn vphaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 47903948, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 3, 180, 128, 204, 244, 218, 2], OperandSize::Qword)
}

