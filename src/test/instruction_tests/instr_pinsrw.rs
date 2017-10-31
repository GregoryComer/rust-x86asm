use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM5)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 237, 40], OperandSize::Dword)
}

#[test]
fn pinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(ESI, 1723717333, Some(OperandSize::Word), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 158, 213, 214, 189, 102, 53], OperandSize::Dword)
}

#[test]
fn pinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM2)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 214, 79], OperandSize::Qword)
}

#[test]
fn pinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 32, 79], OperandSize::Qword)
}

#[test]
fn pinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 210, 115], OperandSize::Dword)
}

#[test]
fn pinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 1154324967, Some(OperandSize::Word), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 129, 231, 153, 205, 68, 67], OperandSize::Dword)
}

#[test]
fn pinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 237, 45], OperandSize::Qword)
}

#[test]
fn pinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RCX, 1084561611, Some(OperandSize::Word), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 129, 203, 24, 165, 64, 6], OperandSize::Qword)
}

