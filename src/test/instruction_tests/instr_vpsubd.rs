use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 250, 215], OperandSize::Dword)
}

#[test]
fn vpsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 1366525982, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 250, 135, 30, 136, 115, 81], OperandSize::Dword)
}

#[test]
fn vpsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 250, 253], OperandSize::Qword)
}

#[test]
fn vpsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1300655153, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 250, 164, 158, 49, 108, 134, 77], OperandSize::Qword)
}

#[test]
fn vpsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 250, 225], OperandSize::Dword)
}

#[test]
fn vpsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 250, 20, 145], OperandSize::Dword)
}

#[test]
fn vpsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 250, 192], OperandSize::Qword)
}

#[test]
fn vpsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1346655170, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 250, 52, 213, 194, 83, 68, 80], OperandSize::Qword)
}

