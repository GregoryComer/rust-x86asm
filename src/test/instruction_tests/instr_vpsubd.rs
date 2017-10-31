use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 250, 226], OperandSize::Dword)
}

#[test]
fn vpsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 818683767, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 250, 52, 205, 119, 31, 204, 48], OperandSize::Dword)
}

#[test]
fn vpsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 250, 210], OperandSize::Qword)
}

#[test]
fn vpsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 250, 4, 150], OperandSize::Qword)
}

#[test]
fn vpsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 250, 212], OperandSize::Dword)
}

#[test]
fn vpsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 250, 52, 215], OperandSize::Dword)
}

#[test]
fn vpsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 250, 253], OperandSize::Qword)
}

#[test]
fn vpsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 250, 49], OperandSize::Qword)
}

