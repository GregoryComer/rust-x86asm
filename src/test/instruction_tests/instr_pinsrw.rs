use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM6)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 243, 72], OperandSize::Dword)
}

#[test]
fn pinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(EBX, 280694115, Some(OperandSize::Word), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 179, 99, 13, 187, 16, 3], OperandSize::Dword)
}

#[test]
fn pinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM6)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 247, 37], OperandSize::Qword)
}

#[test]
fn pinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 11, 112], OperandSize::Qword)
}

#[test]
fn pinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 195, 77], OperandSize::Dword)
}

#[test]
fn pinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1960504391, Some(OperandSize::Word), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 180, 254, 71, 236, 218, 116, 88], OperandSize::Dword)
}

#[test]
fn pinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 234, 72], OperandSize::Qword)
}

#[test]
fn pinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 345253356, Some(OperandSize::Word), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 132, 248, 236, 37, 148, 20, 48], OperandSize::Qword)
}

