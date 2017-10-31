use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 237], OperandSize::Dword)
}

#[test]
fn pminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(ESI, 1299480081, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 134, 17, 126, 116, 77], OperandSize::Dword)
}

#[test]
fn pminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 232], OperandSize::Qword)
}

#[test]
fn pminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(RBX, 1947615968, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 179, 224, 66, 22, 116], OperandSize::Qword)
}

#[test]
fn pminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 196], OperandSize::Dword)
}

#[test]
fn pminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 15], OperandSize::Dword)
}

#[test]
fn pminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 246], OperandSize::Qword)
}

#[test]
fn pminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1458352664, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 172, 193, 24, 178, 236, 86], OperandSize::Qword)
}

