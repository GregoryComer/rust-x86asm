use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 6, 193], OperandSize::Dword)
}

#[test]
fn vphsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1688040014, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 6, 172, 182, 78, 114, 157, 100], OperandSize::Dword)
}

#[test]
fn vphsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 6, 215], OperandSize::Qword)
}

#[test]
fn vphsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDX, 531353211, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 6, 138, 123, 206, 171, 31], OperandSize::Qword)
}

#[test]
fn vphsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 6, 210], OperandSize::Dword)
}

#[test]
fn vphsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 6, 43], OperandSize::Dword)
}

#[test]
fn vphsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 6, 207], OperandSize::Qword)
}

#[test]
fn vphsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1437306922, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 6, 180, 177, 42, 144, 171, 85], OperandSize::Qword)
}

