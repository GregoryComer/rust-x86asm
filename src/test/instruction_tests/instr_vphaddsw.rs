use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 3, 207], OperandSize::Dword)
}

#[test]
fn vphaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 347981168, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 3, 188, 182, 112, 197, 189, 20], OperandSize::Dword)
}

#[test]
fn vphaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 3, 204], OperandSize::Qword)
}

#[test]
fn vphaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 3, 40], OperandSize::Qword)
}

#[test]
fn vphaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 3, 219], OperandSize::Dword)
}

#[test]
fn vphaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 3, 39], OperandSize::Dword)
}

#[test]
fn vphaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 3, 240], OperandSize::Qword)
}

#[test]
fn vphaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 3, 40], OperandSize::Qword)
}

