use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 193], OperandSize::Dword)
}

#[test]
fn movdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 361399347, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 36, 181, 51, 132, 138, 21], OperandSize::Dword)
}

#[test]
fn movdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 241], OperandSize::Qword)
}

#[test]
fn movdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 706451901, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 60, 221, 189, 153, 27, 42], OperandSize::Qword)
}

#[test]
fn movdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 206], OperandSize::Dword)
}

#[test]
fn movdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 4, 254], OperandSize::Dword)
}

#[test]
fn movdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 213], OperandSize::Qword)
}

#[test]
fn movdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectDisplaced(RDI, 1862983277, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 191, 109, 222, 10, 111], OperandSize::Qword)
}

