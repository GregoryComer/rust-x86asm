use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 227], OperandSize::Dword)
}

#[test]
fn movdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 54], OperandSize::Dword)
}

#[test]
fn movdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 236], OperandSize::Qword)
}

#[test]
fn movdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 27], OperandSize::Qword)
}

#[test]
fn movdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 240], OperandSize::Dword)
}

#[test]
fn movdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 656596466, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 164, 91, 242, 221, 34, 39], OperandSize::Dword)
}

#[test]
fn movdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 208], OperandSize::Qword)
}

#[test]
fn movdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectDisplaced(RAX, 167156869, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 152, 133, 156, 246, 9], OperandSize::Qword)
}

