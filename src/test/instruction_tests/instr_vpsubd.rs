use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 250, 234], OperandSize::Dword)
}

#[test]
fn vpsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 250, 2], OperandSize::Dword)
}

#[test]
fn vpsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 250, 199], OperandSize::Qword)
}

#[test]
fn vpsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1622731686, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 250, 188, 131, 166, 235, 184, 96], OperandSize::Qword)
}

#[test]
fn vpsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 250, 254], OperandSize::Dword)
}

#[test]
fn vpsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 2043573662, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 250, 20, 125, 158, 117, 206, 121], OperandSize::Dword)
}

#[test]
fn vpsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 250, 197], OperandSize::Qword)
}

#[test]
fn vpsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1224601734, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 250, 4, 245, 134, 240, 253, 72], OperandSize::Qword)
}

