use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 226], OperandSize::Dword)
}

#[test]
fn paddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(ECX, 1177644737, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 185, 193, 110, 49, 70], OperandSize::Dword)
}

#[test]
fn paddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 250], OperandSize::Qword)
}

#[test]
fn paddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1388614544, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 188, 80, 144, 147, 196, 82], OperandSize::Qword)
}

#[test]
fn paddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 192], OperandSize::Dword)
}

#[test]
fn paddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1053188975, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 172, 67, 111, 99, 198, 62], OperandSize::Dword)
}

#[test]
fn paddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 232], OperandSize::Qword)
}

#[test]
fn paddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1254352636, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 4, 125, 252, 230, 195, 74], OperandSize::Qword)
}

