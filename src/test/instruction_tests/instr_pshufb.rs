use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 240], OperandSize::Dword)
}

#[test]
fn pshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 60, 128], OperandSize::Dword)
}

#[test]
fn pshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 198], OperandSize::Qword)
}

#[test]
fn pshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 12, 130], OperandSize::Qword)
}

#[test]
fn pshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 202], OperandSize::Dword)
}

#[test]
fn pshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1099816072, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 52, 221, 136, 220, 141, 65], OperandSize::Dword)
}

#[test]
fn pshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 222], OperandSize::Qword)
}

#[test]
fn pshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 20, 75], OperandSize::Qword)
}

