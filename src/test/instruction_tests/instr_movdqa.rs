use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 232], OperandSize::Dword)
}

#[test]
fn movdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 20, 120], OperandSize::Dword)
}

#[test]
fn movdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 215], OperandSize::Qword)
}

#[test]
fn movdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RBX, 1892735796, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 155, 52, 219, 208, 112], OperandSize::Qword)
}

#[test]
fn movdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 197], OperandSize::Dword)
}

#[test]
fn movdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectDisplaced(EBX, 1900903748, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 171, 68, 125, 77, 113], OperandSize::Dword)
}

#[test]
fn movdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 229], OperandSize::Qword)
}

#[test]
fn movdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectDisplaced(RSI, 2028321656, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 134, 120, 187, 229, 120], OperandSize::Qword)
}

