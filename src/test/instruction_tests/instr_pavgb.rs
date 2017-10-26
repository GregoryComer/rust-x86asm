use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 220], OperandSize::Dword)
}

#[test]
fn pavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 13816821, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 132, 95, 245, 211, 210, 0], OperandSize::Dword)
}

#[test]
fn pavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 247], OperandSize::Qword)
}

#[test]
fn pavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RAX, 888126934, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 168, 214, 189, 239, 52], OperandSize::Qword)
}

#[test]
fn pavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 206], OperandSize::Dword)
}

#[test]
fn pavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 62], OperandSize::Dword)
}

#[test]
fn pavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 242], OperandSize::Qword)
}

#[test]
fn pavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 44, 114], OperandSize::Qword)
}

