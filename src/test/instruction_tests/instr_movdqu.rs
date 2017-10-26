use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 218], OperandSize::Dword)
}

#[test]
fn movdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 12, 215], OperandSize::Dword)
}

#[test]
fn movdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 231], OperandSize::Qword)
}

#[test]
fn movdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 41], OperandSize::Qword)
}

#[test]
fn movdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 227], OperandSize::Dword)
}

#[test]
fn movdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectDisplaced(ESI, 1351633238, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 166, 86, 73, 144, 80], OperandSize::Dword)
}

#[test]
fn movdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 231], OperandSize::Qword)
}

#[test]
fn movdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectDisplaced(RBX, 120898406, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 131, 102, 195, 52, 7], OperandSize::Qword)
}

