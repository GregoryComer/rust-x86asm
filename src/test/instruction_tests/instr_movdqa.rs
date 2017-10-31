use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 236], OperandSize::Dword)
}

#[test]
fn movdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 49], OperandSize::Dword)
}

#[test]
fn movdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 220], OperandSize::Qword)
}

#[test]
fn movdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDI, 2006542631, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 183, 39, 105, 153, 119], OperandSize::Qword)
}

#[test]
fn movdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 234], OperandSize::Dword)
}

#[test]
fn movdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1691444743, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 52, 117, 7, 102, 209, 100], OperandSize::Dword)
}

#[test]
fn movdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 254], OperandSize::Qword)
}

#[test]
fn movdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 322729142, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 164, 211, 182, 116, 60, 19], OperandSize::Qword)
}

