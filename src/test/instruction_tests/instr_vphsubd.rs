use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 6, 232], OperandSize::Dword)
}

#[test]
fn vphsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 1000430417, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 6, 147, 81, 91, 161, 59], OperandSize::Dword)
}

#[test]
fn vphsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 6, 249], OperandSize::Qword)
}

#[test]
fn vphsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RAX, 1446307004, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 6, 152, 188, 228, 52, 86], OperandSize::Qword)
}

#[test]
fn vphsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 6, 223], OperandSize::Dword)
}

#[test]
fn vphsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1281620208, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 6, 12, 181, 240, 248, 99, 76], OperandSize::Dword)
}

#[test]
fn vphsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 6, 239], OperandSize::Qword)
}

#[test]
fn vphsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 6, 52, 145], OperandSize::Qword)
}

