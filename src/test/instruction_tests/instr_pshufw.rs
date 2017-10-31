use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 226, 49], OperandSize::Word)
}

#[test]
fn pshufw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(BP, 213, Some(OperandSize::Qword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 158, 213, 0, 39], OperandSize::Word)
}

#[test]
fn pshufw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 237, 74], OperandSize::Dword)
}

#[test]
fn pshufw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 44, 67, 107], OperandSize::Dword)
}

#[test]
fn pshufw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 233, 116], OperandSize::Qword)
}

#[test]
fn pshufw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 12, 130, 119], OperandSize::Qword)
}

